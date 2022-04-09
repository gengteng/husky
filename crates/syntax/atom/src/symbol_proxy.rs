use entity_route::{EntityRouteKind, *};
use entity_route_query::EntityRouteQueryGroup;
use entity_syntax::TyKind;
use file::FilePtr;
use text::{Row, TextRange};
use word::{ContextualIdentifier, CustomIdentifier, RootIdentifier};

use super::*;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ident: CustomIdentifier,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn var(ident: word::CustomIdentifier, init_row: Row) -> Self {
        Self {
            ident: ident.into(),
            kind: SymbolKind::Variable { init_row },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SymbolKind {
    Scope(EntityRouteKind),
    Variable { init_row: Row },
    Unrecognized(CustomIdentifier),
    ThisData { ty: Option<EntityRoutePtr> },
    ThisType { ty: Option<EntityRoutePtr> },
}

#[derive(Clone, Copy)]
pub struct SymbolProxy<'a> {
    pub opt_package_main: Option<FilePtr>,
    pub db: &'a dyn EntityRouteQueryGroup,
    pub this_ty: Option<EntityRoutePtr>,
    pub symbols: &'a fold::LocalStack<Symbol>,
}

impl<'a> SymbolProxy<'a> {
    pub fn builtin_type_atom(
        &self,
        ident: RootIdentifier,
        generics: Vec<GenericArgument>,
        tail: TextRange,
    ) -> Atom {
        let scope = EntityRoute::new_builtin(ident.into(), generics);
        let kind = AtomKind::EntityRoute {
            route: self.db.intern_scope(scope),
            kind: EntityKind::Type(match ident {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::F32
                | RootIdentifier::B32
                | RootIdentifier::B64
                | RootIdentifier::Bool => TyKind::Primitive,
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => TyKind::Other,
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Fp => TyKind::Other,
                RootIdentifier::Fn => TyKind::Other,
                RootIdentifier::FnMut => TyKind::Other,
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::Type => todo!(),
                RootIdentifier::Datasets => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
            }),
        };
        Atom::new(tail, kind)
    }

    pub fn resolve_symbol_kind(
        &self,
        ident: Identifier,
        range: TextRange,
    ) -> AtomResult<SymbolKind> {
        match ident {
            Identifier::Builtin(ident) => Ok(SymbolKind::Scope(ident.into())),
            Identifier::Contextual(ident) => match ident {
                ContextualIdentifier::Input => Ok(SymbolKind::Scope(EntityRouteKind::Input {
                    main: self
                        .opt_package_main
                        .ok_or(error!("can't use implicit without main", range))?,
                })),
                ContextualIdentifier::ThisData => Ok(SymbolKind::ThisData { ty: self.this_ty }),
                ContextualIdentifier::ThisType => Ok(SymbolKind::ThisType { ty: self.this_ty }),
            },
            Identifier::Custom(ident) => Ok(
                if let Some(symbol) = self.symbols.find(|symbol| symbol.ident == ident.into()) {
                    symbol.kind
                } else {
                    SymbolKind::Unrecognized(ident)
                },
            ),
        }
    }

    // fn resolve_subscope(
    //     &self,
    //     parent_scope: EntityRoute,
    //     subscope_ident: CustomIdentifier,
    // ) -> Option<EntityRoutePtr> {
    //     self.db.subscope(
    //         self.db.intern_scope(parent_scope),
    //         subscope_ident,
    //         Vec::new(),
    //     )
    // }
}