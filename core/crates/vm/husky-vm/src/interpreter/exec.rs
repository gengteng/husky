mod exec_call;
mod exec_condition_flow;
mod exec_interpret_call;
mod exec_loop;
mod exec_opr_opn;
mod exec_pattern_match;

use crate::{history::HistoryEntry, *};
use check_utils::{should, should_eq};
use colored::Colorize;
use husky_entity_route::EntityRouteKind;
use path_utils::get_relative_path;
use print_utils::{msg_once, p, ps};
use std::iter::zip;
use word::RootIdentifier;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(crate) fn exec_all(&mut self, sheet: &InstructionSheet, mode: Mode) -> VMControl<'eval> {
        for ins in &sheet.instructions {
            if self.vm_config.verbose {
                println!(
                    "{} {}:{}",
                    "exec".red(),
                    get_relative_path(&ins.src.file())
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .green(),
                    format!(
                        "{:?} .. {:?}",
                        ins.src.text_range().start,
                        ins.src.text_range().end
                    )
                    .bright_yellow(),
                )
            }
            let control = match ins.variant {
                InstructionVariant::PushVariable {
                    binding,
                    stack_idx,
                    range,
                    ty,
                    varname,
                } => {
                    let value = self.stack.push_variable(stack_idx, binding);
                    match mode {
                        Mode::Fast => (),
                        Mode::TrackMutation => match binding {
                            Binding::TempRefMut => {
                                self.record_mutation(stack_idx, varname, ins.src.file(), range, ty)
                            }
                            _ => (),
                        },
                        Mode::TrackHistory => {
                            should_eq!(ty, value.any_ref().__ty_dyn());
                            self.history.write(
                                ins,
                                HistoryEntry::PureExpr {
                                    result: Ok(value.eval()),
                                },
                            )
                        }
                    }
                    VMControl::None
                }
                InstructionVariant::PushPrimitiveLiteral { value, explicit } => {
                    self.stack.push(value.into());
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            if explicit {
                                self.history.write(
                                    ins,
                                    HistoryEntry::PureExpr {
                                        result: Ok(self.stack.eval_top()),
                                    },
                                )
                            }
                        }
                    }
                    VMControl::None
                }
                InstructionVariant::PushEnumKindLiteral(entity_kind) => {
                    self.stack.push(__TempValue::Copyable(entity_kind.into()));
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(self.stack.eval_top()),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionVariant::CallSpecificRoutine {
                    linkage,
                    output_ty: ty,
                } => {
                    let control = self.call_specific_routine(linkage, ty).into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: match control {
                                    VMControl::Err(ref e) => Err(e.clone().into()),
                                    _ => Ok(self.stack.eval_top()),
                                },
                            },
                        ),
                    }
                    control
                }
                InstructionVariant::CallGenericRoutine {
                    output_ty,
                    linkage: __Linkage,
                } => {
                    let control = self.call_generic_transfer(output_ty, __Linkage).into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: match control {
                                    VMControl::Err(ref e) => Err(e.clone().into()),
                                    _ => Ok(self.stack.eval_top()),
                                },
                            },
                        ),
                    }
                    control
                }
                InstructionVariant::OprOpn { opn, .. } => {
                    // sheet.variable_stack.compare_with_vm_stack(&self.stack);
                    self.exec_opr_opn(opn, mode, ins).into()
                }
                InstructionVariant::CallInterpreted {
                    routine_uid: routine,
                    nargs, // including this
                    has_this,
                    output_ty,
                } => {
                    let instruction_sheet = self.db.entity_opt_instruction_sheet_by_uid(routine);
                    let result =
                        self.call_interpreted(&instruction_sheet.unwrap(), nargs, has_this);
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            let result = match result {
                                Ok(()) => Ok(self.stack.eval_top()),
                                Err(ref e) => Err(e.clone()),
                            };
                            self.history.write(ins, HistoryEntry::PureExpr { result })
                        }
                    };
                    result.into()
                }
                InstructionVariant::NewVirtualStruct { ty, ref fields } => {
                    let value_result = self.new_virtual_struct(ty, fields);
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            let output = self.stack.eval_top();
                            should_eq!(output.any_ref().__ty_dyn(), ty);
                            self.history
                                .write(ins, HistoryEntry::PureExpr { result: Ok(output) })
                        }
                    }
                    VMControl::None
                }
                InstructionVariant::Return { output_ty } => {
                    let return_value = self.stack.pop().into_eval();
                    msg_once!("ugly");
                    if output_ty.kind
                        != (EntityRouteKind::Root {
                            ident: RootIdentifier::DatasetType,
                        })
                    {
                        should_eq!(output_ty, return_value.any_ref().__ty_dyn());
                    }
                    VMControl::Return(return_value)
                }
                InstructionVariant::Loop {
                    ref body,
                    loop_kind,
                } => {
                    let control = match mode {
                        Mode::Fast => self.exec_loop_fast(loop_kind, body).into(),
                        Mode::TrackMutation => {
                            self.exec_loop_tracking_mutation(loop_kind, body).into()
                        }
                        Mode::TrackHistory => {
                            self.save_snapshot("Loop".to_string());
                            let control = self.exec_loop_tracking_mutation(loop_kind, body).into();
                            let (snapshot, mutations) = self.collect_block_mutations();
                            self.history.write(
                                ins,
                                HistoryEntry::loop_entry(
                                    loop_kind,
                                    &control,
                                    snapshot,
                                    body.clone(),
                                    mutations,
                                ),
                            );
                            control
                        }
                    };
                    control
                }
                InstructionVariant::BreakIfFalse => {
                    let control = if !self.stack.pop().take_copyable().to_bool() {
                        VMControl::Break
                    } else {
                        VMControl::None
                    };
                    control
                }
                InstructionVariant::FieldAccessInterpreted {
                    field_idx,
                    field_binding,
                } => {
                    let this = self.stack.pop();
                    self.stack
                        .push(this.field(field_idx as usize, field_binding));
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(self.stack.eval_top()),
                            },
                        ),
                    };
                    VMControl::None
                }
                InstructionVariant::Assert => {
                    let is_condition_satisfied = self.stack.pop().take_copyable().to_bool();
                    if !is_condition_satisfied {
                        VMControl::Err(EvalError::Normal {
                            message: format!("assert failure"),
                        })
                    } else {
                        VMControl::None
                    }
                }
                InstructionVariant::Break => {
                    if mode == Mode::TrackHistory {
                        self.history.write(ins, HistoryEntry::Break)
                    }
                    VMControl::Break
                }
                InstructionVariant::ConditionFlow { ref branches } => {
                    self.exec_condition_flow(sheet, ins, branches, mode)
                }
                InstructionVariant::PatternMatch { ref branches } => {
                    self.exec_pattern_matching(sheet, ins, branches, mode)
                } // InstructionVariant::NewXmlFromValue { ty } => {
                  //     let visual_props = self.db.visualize(ty, self.stack.pop().any_temp_ref());
                  //     self.stack
                  //         .push(__TempValue::OwnedEval(OwnedValue::new(visual_props)));
                  //     VMControl::None
                  // }
                  // InstructionVariant::NewXmlFromTag {
                  //     tag_kind,
                  //     ref props,
                  //     n_child_expr,
                  // } => {
                  //     if n_child_expr > 0 {
                  //         todo!()
                  //     }
                  //     let arguments = self.stack.drain(props.len().try_into().unwrap());

                  //     match mode {
                  //         Mode::Fast => (),
                  //         Mode::TrackMutation => todo!(),
                  //         Mode::TrackHistory => todo!(),
                  //     }
                  //     VMControl::None
                  // }
            };
            match control {
                VMControl::None => (),
                VMControl::Break | VMControl::Return(_) | VMControl::Err(_) => return control,
            }
        }
        VMControl::None
    }

    pub(crate) fn eval_linkage(
        &mut self,
        linkage: __Linkage,
        output_ty: EntityRoutePtr,
    ) -> __EvalValueResult<'eval> {
        match linkage {
            __Linkage::Member { .. } => todo!(),
            __Linkage::SpecificTransfer(linkage) => {
                let mut arguments = self.stack.drain(linkage.nargs).collect::<Vec<_>>();
                should_eq!(self.stack.len(), 0);
                linkage.eval(arguments)
            }
            __Linkage::GenericTransfer(__Linkage) => {
                let mut arguments = self.stack.drain(__Linkage.nargs).collect::<Vec<_>>();
                should_eq!(self.stack.len(), 0);
                Ok((__Linkage.call)(output_ty, &mut arguments).into_eval())
            }
            __Linkage::Model(_) => todo!(),
        }
    }
}