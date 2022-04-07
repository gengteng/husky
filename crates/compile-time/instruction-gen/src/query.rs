use check_utils::should_eq;
use file::FilePtr;
use linkage_table::HasFpTable;
use pack_semantics::PackQueryGroup;
use vm::{EvalValue, RoutineFp, StackValue, VMResult};

use crate::*;

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup: EntityQueryGroup + PackQueryGroup + HasFpTable {
    fn entity_instruction_sheet(&self, route: EntityRoutePtr) -> Arc<InstructionSheet>;
    fn memb_routine_instruction_sheet(
        &self,
        ty: EntityRoutePtr,
        memb_ident: CustomIdentifier,
    ) -> Arc<InstructionSheet>;
    fn dataset_config_instruction_sheet(&self, pack_main: FilePtr) -> Arc<InstructionSheet>;
    fn virtual_vec_memb_routine_fps(&self) -> Arc<IdentMap<RoutineFp>>;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    route: EntityRoutePtr,
) -> Arc<InstructionSheet> {
    let entity_defn = db
        .opt_entity_defn(route)
        .unwrap()
        .expect("expect no builtin");
    match entity_defn.kind() {
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern { .. } => todo!(),
        EntityDefnVariant::Func {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_decl(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityDefnVariant::Proc {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_impr(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityDefnVariant::Ty(_) => todo!(),
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Builtin => {
            p!(route.ident());
            todo!()
        }
        EntityDefnVariant::EnumVariant(_) => todo!(),
    }
}

fn memb_routine_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    ty: EntityRoutePtr,
    memb_ident: CustomIdentifier,
) -> Arc<InstructionSheet> {
    let entity_defn = db.opt_entity_defn(ty).unwrap().unwrap();
    match entity_defn.kind() {
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Module {} => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern {} => todo!(),
        EntityDefnVariant::Func {
            input_placeholders,
            output,
            stmts,
        } => todo!(),
        EntityDefnVariant::Proc {
            input_placeholders,
            output,
            stmts,
        } => todo!(),
        EntityDefnVariant::Ty(ty) => match ty.kind {
            TyDefnVariant::Enum { ref variants } => todo!(),
            TyDefnVariant::Struct {
                ref memb_vars,
                ref memb_routines,
            } => {
                let memb_routine = memb_routines.get(memb_ident).unwrap();
                let inputs = memb_routine
                    .input_placeholders
                    .iter()
                    .map(|input_placeholder| input_placeholder.ident)
                    .collect();
                match memb_routine.kind {
                    MembRoutineKind::Func { ref stmts } => {
                        InstructionSheetBuilder::new_decl(db, inputs, stmts, true)
                    }
                    MembRoutineKind::Proc { ref stmts } => {
                        InstructionSheetBuilder::new_impr(db, inputs, stmts, true)
                    }
                }
            }
            TyDefnVariant::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
        },
        EntityDefnVariant::Builtin => todo!(),
        EntityDefnVariant::EnumVariant(_) => todo!(),
    }
}

fn dataset_config_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    pack_main: FilePtr,
) -> Arc<InstructionSheet> {
    let pack = db.pack(pack_main).unwrap();
    InstructionSheetBuilder::new_decl(db, vec![], &pack.config.dataset.stmts, false)
}

fn virtual_vec_memb_routine_fps(db: &dyn InstructionGenQueryGroup) -> Arc<IdentMap<RoutineFp>> {
    let mut memb_routine_fps = IdentMap::default();
    memb_routine_fps.insert_new(
        db.intern_word("len").opt_custom().unwrap(),
        RoutineFp {
            call: virtual_vec_len,
            nargs: 1,
        },
    );
    memb_routine_fps.insert_new(
        db.intern_word("push").opt_custom().unwrap(),
        RoutineFp {
            call: virtual_vec_push,
            nargs: 2,
        },
    );
    memb_routine_fps.insert_new(
        db.intern_word("pop").opt_custom().unwrap(),
        RoutineFp {
            call: virtual_vec_pop,
            nargs: 1,
        },
    );
    memb_routine_fps.insert_new(
        db.intern_word("first").opt_custom().unwrap(),
        RoutineFp {
            call: virtual_vec_first,
            nargs: 1,
        },
    );
    memb_routine_fps.insert_new(
        db.intern_word("last").opt_custom().unwrap(),
        RoutineFp {
            call: virtual_vec_last,
            nargs: 1,
        },
    );
    Arc::new(memb_routine_fps)
}

fn virtual_vec_len<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let virtual_vec: &Vec<EvalValue<'eval>> = values[0].downcast_ref();
    let len: i32 = virtual_vec.len().try_into().unwrap();
    Ok(StackValue::Primitive(len.into()))
}

fn virtual_vec_push<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 2);
    let element = values[1].into_eval();
    let virtual_vec: &mut Vec<EvalValue<'eval>> = values[0].downcast_mut();
    virtual_vec.push(element);
    Ok(StackValue::Primitive(().into()))
}

fn virtual_vec_pop<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn virtual_vec_first<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn virtual_vec_last<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}