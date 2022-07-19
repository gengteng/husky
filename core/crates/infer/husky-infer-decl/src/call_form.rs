mod output;
mod parameter;
mod variadic;

pub use output::*;
pub use parameter::*;
pub use variadic::*;

use defn_head::*;
use fold::LocalStack;
use husky_atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use husky_implement::{Implementable, ImplementationContext};
use husky_instantiate::InstantiationContext;
use map_collect::MapCollect;
use print_utils::{msg_once, p};
use static_defn::{EntityStaticDefnVariant, StaticParameter};
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CallFormDecl {
    pub base_route: EntityRoutePtr,
    pub opt_this_liason: Option<ParameterLiason>,
    pub spatial_parameters: IdentDict<SpatialParameter>,
    pub primary_parameters: IdentDict<ParameterDecl>,
    pub variadic_template: VariadicTemplate,
    pub keyword_parameters: IdentDict<ParameterDecl>,
    pub output: OutputDecl,
    pub is_lazy: bool,
}

impl CallFormDecl {
    pub(crate) fn from_ast(route: EntityRoutePtr, ast: &Ast) -> Arc<Self> {
        msg_once!("variadics");
        match ast.variant {
            AstVariant::CallFormDefnHead {
                ident,
                paradigm,
                spatial_parameters: ref generic_parameters,
                ref parameters,
                output_ty,
                output_liason,
                opt_this_liason,
            } => Arc::new(CallFormDecl {
                base_route: route,
                opt_this_liason,
                spatial_parameters: generic_parameters.clone(),
                primary_parameters: parameters
                    .iter()
                    .map(|parameter| parameter.into())
                    .collect(),
                output: OutputDecl {
                    ty: output_ty.route,
                    liason: output_liason,
                },
                keyword_parameters: Default::default(),
                variadic_template: VariadicTemplate::None,
                is_lazy: paradigm.is_lazy(),
            }),
            _ => todo!(),
        }
    }

    pub fn from_static(
        route: EntityRoutePtr,
        symbol_context: &mut dyn AtomContext,
        defn: &EntityStaticDefn,
    ) -> Arc<Self> {
        match defn.variant {
            EntityStaticDefnVariant::Method {
                this_liason,
                parameters,
                output_ty,
                output_liason,
                spatial_parameters,
                method_static_defn_kind: method_kind,
                ..
            } => {
                let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
                Arc::new(Self {
                    base_route: route,
                    opt_this_liason: Some(this_liason),
                    primary_parameters: parameters
                        .map(|input| ParameterDecl::from_static(symbol_context, input)),
                    output: OutputDecl {
                        liason: output_liason,
                        ty: output_ty,
                    },
                    spatial_parameters: spatial_parameters.map(|static_generic_placeholder| {
                        SpatialParameter::from_static(
                            symbol_context.entity_syntax_db(),
                            static_generic_placeholder,
                        )
                    }),
                    is_lazy: false,
                    variadic_template: todo!(),
                    keyword_parameters: todo!(),
                })
            }
            _ => panic!(""),
        }
    }

    pub fn ident(&self) -> CustomIdentifier {
        self.base_route.ident().custom()
    }

    pub fn nargs(&self) -> u8 {
        let nargs0: u8 = self.primary_parameters.len().try_into().unwrap();
        nargs0 + self.opt_this_liason.map(|_| 1u8).unwrap_or(0u8)
    }

    pub fn this_liason(&self) -> ParameterLiason {
        self.opt_this_liason.unwrap()
    }
}

impl Instantiable for CallFormDecl {
    type Target = Arc<Self>;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        Arc::new(Self {
            base_route: self.base_route.instantiate(ctx).take_entity_route(),
            opt_this_liason: self.opt_this_liason,
            spatial_parameters: self
                .spatial_parameters
                .iter()
                .filter_map(|parameter| parameter.instantiate(ctx))
                .collect(),
            primary_parameters: self
                .primary_parameters
                .map(|parameter| parameter.instantiate(ctx)),
            output: self.output.instantiate(ctx),
            keyword_parameters: self
                .primary_parameters
                .map(|parameter| parameter.instantiate(ctx)),
            variadic_template: self.variadic_template.instantiate(ctx),
            is_lazy: self.is_lazy,
        })
    }
}

impl Implementable for CallFormDecl {
    type Target = Arc<Self>;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target {
        Arc::new(Self {
            base_route: self.base_route.implement(ctx),
            opt_this_liason: self.opt_this_liason,
            primary_parameters: self
                .primary_parameters
                .map(|parameter| parameter.implement(ctx)),
            keyword_parameters: self
                .keyword_parameters
                .map(|parameter| parameter.implement(ctx)),
            output: self.output.implement(ctx),
            spatial_parameters: self.spatial_parameters.clone(),
            is_lazy: self.is_lazy,
            variadic_template: self.variadic_template.implement(ctx),
        })
    }
}

pub(crate) fn call_form_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferQueryResultArc<CallFormDecl> {
    let locus = db.entity_locus(route)?;
    return match locus {
        EntityLocus::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Function { .. } => {
                routine_decl_from_static(db, vec![], route, static_defn)
            }
            EntityStaticDefnVariant::Ty { .. } => match db.ty_decl(route)?.opt_type_call {
                Some(ref ty_call) => ty_call.clone(),
                None => return Err(query_error!(format!("no type call for {:?}", route))),
            },
            _ => panic!(),
        }),
        EntityLocus::WithinBuiltinModule => todo!(),
        EntityLocus::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.variant {
                AstVariant::CallFormDefnHead { .. } => Ok(CallFormDecl::from_ast(route, ast)),
                // type constructor
                AstVariant::TypeDefnHead { .. } => {
                    let ty_decl = db.ty_decl(route)?;
                    Ok(ty_decl.opt_type_call.clone().expect("todo"))
                }
                _ => panic!(),
            }
        }
        EntityLocus::Module { file: file_id } => todo!(),
        EntityLocus::Input { .. } => todo!(),
        EntityLocus::StaticTypeMember => todo!(),
        EntityLocus::StaticTypeAsTraitMember => todo!(),
    };
}

pub(crate) fn routine_decl_from_static(
    db: &dyn DeclQueryGroup,
    mut symbols: Vec<Symbol>,
    route: EntityRoutePtr,
    static_defn: &EntityStaticDefn,
) -> Arc<CallFormDecl> {
    match static_defn.variant {
        EntityStaticDefnVariant::Function {
            ref spatial_parameters,
            ref parameters,
            output_ty,
            output_liason,
            ref linkage,
            ref variadic_template,
        } => {
            let generic_parameters = db.generic_parameters_from_static(spatial_parameters);
            symbols.extend(db.symbols_from_generic_parameters(&generic_parameters));
            let mut symbol_context = AtomContextStandalone {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: (&symbols as &[Symbol]).into(),
                kind: AtomContextKind::Normal,
            };
            let parameters = parameters.map(|parameter| ParameterDecl {
                ty: symbol_context.parse_entity_route(parameter.ty).unwrap(),
                liason: parameter.liason,
                ident: db.custom_ident(parameter.name),
            });
            let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
            msg_once!("todo: keyword parameters");
            Arc::new(CallFormDecl {
                base_route: route,
                spatial_parameters: generic_parameters,
                primary_parameters: parameters,
                output: OutputDecl {
                    liason: output_liason,
                    ty: output_ty,
                },
                keyword_parameters: Default::default(),
                variadic_template: VariadicTemplate::from_static(
                    &mut symbol_context,
                    variadic_template,
                ),
                opt_this_liason: None,
                is_lazy: false,
            })
        }
        _ => panic!(),
    }
}

// pub(crate) fn model_decl_from_static(
//     db: &dyn DeclQueryGroup,
//     mut symbols: Vec<Symbol>,
//     route: EntityRoutePtr,
//     static_defn: &EntityStaticDefn,
// ) -> Arc<FunctionDecl> {
//     match static_defn.variant {
//         EntityStaticDefnVariant::Model {
//             spatial_parameters: ref generic_parameters,
//             ref parameters,
//             output_ty,
//             output_liason,
//             ..
//         } => {
//             let generic_parameters = db.generic_parameters_from_static(generic_parameters);
//             symbols.extend(db.symbols_from_generic_parameters(&generic_parameters));
//             let mut symbol_context = AtomContextStandalone {
//                 opt_package_main: None,
//                 db: db.upcast(),
//                 opt_this_ty: None,
//                 opt_this_contract: None,
//                 symbols: (&symbols as &[Symbol]).into(),
//                 kind: AtomContextKind::Normal,
//             };
//             let parameters = parameters.map(|parameter| ParameterDecl {
//                 ty: symbol_context.parse_entity_route(parameter.ty).unwrap(),
//                 liason: parameter.liason,
//                 ident: db.custom_ident(parameter.name),
//             });
//             let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
//             msg_once!("todo: keyword parameters");
//             Arc::new(FunctionDecl {
//                 route,
//                 spatial_parameters: generic_parameters,
//                 primary_parameters: parameters,
//                 output: OutputDecl {
//                     liason: output_liason,
//                     ty: output_ty,
//                 },
//                 keyword_parameters: Default::default(),
//             })
//         }
//         _ => panic!(),
//     }
// }