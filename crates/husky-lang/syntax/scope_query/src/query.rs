use crate::{error::scope_err, *};
use file::FileId;
use interner::InternId;
use scope::*;

use common::*;

use word::{CustomIdentifier, Identifier, ReservedIdentifier, WordId};

use fold::FoldStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait ScopeSalsaQueryGroup: token::TokenQueryGroup + InternScope {
    fn subscope_table(&self, scope_id: ScopeId) -> ScopeResultArc<SubscopeTable>;

    fn subscope_ids(&self, scope_id: ScopeId) -> Arc<Vec<ScopeId>>;

    fn scope_kind(&self, scope_id: ScopeId) -> ScopeKind;

    fn scope_kind_from_route(&self, route: ScopeRoute) -> ScopeKind;

    fn scope_source(&self, scope_id: ScopeId) -> ScopeResult<ScopeSource>;
}

fn subscope_table(
    this: &dyn ScopeSalsaQueryGroup,
    scope_id: ScopeId,
) -> ScopeResultArc<SubscopeTable> {
    Ok(Arc::new(match this.scope_source(scope_id)? {
        ScopeSource::Builtin(data) => SubscopeTable::builtin(this, data),
        ScopeSource::WithinModule {
            file_id,
            token_group_index,
        } => {
            let text = this.tokenized_text(file_id)?;
            let item = text.fold_iter(token_group_index).next().unwrap();
            if let Some(children) = item.children {
                SubscopeTable::parse(file_id, children)
            } else {
                SubscopeTable::empty()
            }
        }
        ScopeSource::Module { file_id } => {
            let text = this.tokenized_text(file_id)?;
            SubscopeTable::parse(file_id, text.fold_iter(0))
        }
        ScopeSource::WithinBuiltinModule => todo!(),
    }))
}

fn subscope_ids(this: &dyn ScopeSalsaQueryGroup, scope_id: ScopeId) -> Arc<Vec<ScopeId>> {
    Arc::new(this.subscope_table(scope_id).map_or(Vec::new(), |table| {
        table
            .subscopes(scope_id)
            .into_iter()
            .map(|scope| this.intern_scope(scope))
            .collect()
    }))
}

fn scope_kind(this: &dyn ScopeSalsaQueryGroup, scope: ScopeId) -> ScopeKind {
    this.scope_kind_from_route(scope.route)
}

fn scope_kind_from_route(this: &dyn ScopeSalsaQueryGroup, route: ScopeRoute) -> ScopeKind {
    match route {
        ScopeRoute::Reserved { ident } => match ident {
            ReservedIdentifier::Void
            | ReservedIdentifier::I32
            | ReservedIdentifier::F32
            | ReservedIdentifier::B32
            | ReservedIdentifier::B64
            | ReservedIdentifier::Bool
            | ReservedIdentifier::Vector
            | ReservedIdentifier::Tuple
            | ReservedIdentifier::Fp
            | ReservedIdentifier::Array => ScopeKind::Type,
            ReservedIdentifier::Fn | ReservedIdentifier::FnMut | ReservedIdentifier::FnOnce => {
                ScopeKind::Trait
            }
            ReservedIdentifier::Debug
            | ReservedIdentifier::Std
            | ReservedIdentifier::Core
            | ReservedIdentifier::Dataset => ScopeKind::Module,
            ReservedIdentifier::Input => ScopeKind::Feature,
        },
        ScopeRoute::Package { .. } => ScopeKind::Module,
        ScopeRoute::ChildScope { parent, ident } => this
            .subscope_table(parent)
            .unwrap()
            .scope_kind(ident)
            .unwrap(),
    }
}

fn scope_source(this: &dyn ScopeSalsaQueryGroup, scope: ScopeId) -> ScopeResult<ScopeSource> {
    Ok(match scope.route {
        ScopeRoute::Reserved { ident } => match ident {
            ReservedIdentifier::Void => todo!(),
            ReservedIdentifier::I32 => todo!(),
            ReservedIdentifier::F32 => todo!(),
            ReservedIdentifier::B32 => todo!(),
            ReservedIdentifier::B64 => todo!(),
            ReservedIdentifier::Bool => todo!(),
            ReservedIdentifier::Vector => todo!(),
            ReservedIdentifier::Tuple => todo!(),
            ReservedIdentifier::Debug => todo!(),
            ReservedIdentifier::Std => todo!(),
            ReservedIdentifier::Core => todo!(),
            ReservedIdentifier::Fp => todo!(),
            ReservedIdentifier::Fn => todo!(),
            ReservedIdentifier::FnMut => todo!(),
            ReservedIdentifier::FnOnce => todo!(),
            ReservedIdentifier::Array => todo!(),
            ReservedIdentifier::Input => todo!(),
            ReservedIdentifier::Dataset => dataset::SCOPE_DATA,
        }
        .into(),
        ScopeRoute::Package { main, .. } => ScopeSource::Module { file_id: main },
        ScopeRoute::ChildScope { parent, ident } => {
            this.subscope_table(parent)?.scope_source(ident)?
        }
    })
}

pub struct ModuleFromFileError {
    pub rule_broken: ModuleFromFileRule,
}

pub enum ModuleFromFileRule {
    PackageNameShouldBeIdentifier,
    PackageRootShouldHaveFileName,
    FileShouldExist,
    FileShouldHaveExtensionHSK,
}

pub trait ScopeQueryGroup: ScopeSalsaQueryGroup + InternScope {
    fn subscope(
        &self,
        parent_scope: ScopeId,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> Option<Scope> {
        if self
            .subscope_table(parent_scope)
            .map_or(false, |table| table.has_subscope(ident, &generics))
        {
            Some(Scope::child_scope(parent_scope, ident, generics))
        } else {
            None
        }
    }

    fn all_modules(&self) -> Vec<PackageOrModule>
    where
        Self: Sized,
    {
        self.all_main_files()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn module_iter(&self) -> std::vec::IntoIter<PackageOrModule>
    where
        Self: Sized,
    {
        self.all_modules().into_iter()
    }

    fn collect_modules(&self, id: FileId) -> Vec<PackageOrModule>
    where
        Self: Sized,
    {
        if let Ok(module) = self.module_from_file_id(id) {
            let mut modules = vec![module];
            self.subscope_table(module.scope()).ok().map(|table| {
                modules.extend(
                    table
                        .submodule_idents()
                        .into_iter()
                        .filter_map(|ident| {
                            self.submodule_file_id(id, ident)
                                .map_or(None, |id| Some(self.collect_modules(id)))
                        })
                        .flatten(),
                );
            });
            modules
        } else {
            vec![]
        }
    }

    fn module_from_file_id(&self, id: FileId) -> ScopeResult<PackageOrModule> {
        let path: PathBuf = (*id).into();
        if !self.file_exists(id) {
            scope_err!(format!("file didn't exist"))?
        } else if path_has_file_name(&path, "main.hsk") {
            if let Some(package_name) = path_parent_file_name_str(&path) {
                if let WordId::Identifier(Identifier::Custom(ident)) =
                    self.word_interner().intern(package_name)
                {
                    Ok(PackageOrModule {
                        scope_id: self.intern_scope(Scope::package(id, ident)),
                    })
                } else {
                    scope_err!(format!("package name should be identifier"))?
                }
            } else {
                scope_err!(format!("package root should have filename"))?
            }
        } else if path_has_file_name(&path, "mod.hsk") {
            todo!()
        } else if path_has_extension(&path, "hsk") {
            let maybe_main_path = path.with_file_name("main.hsk");
            if maybe_main_path.exists() {
                let _parent =
                    self.module_from_file_id(self.file_id(path.with_file_name("mod.hsk")));
                todo!()
            } else {
                todo!()
            }
        } else {
            scope_err!(format!("file should have extension .hsk"))?
        }
    }

    fn module_to_file_id(&self, module: PackageOrModule) -> ScopeResult<FileId> {
        Ok(match self.scope_source(module.scope())? {
            ScopeSource::Builtin(_) => panic!(),
            ScopeSource::WithinModule { file_id, .. } => file_id,
            ScopeSource::Module { file_id } => file_id,
            ScopeSource::WithinBuiltinModule => todo!(),
        })
    }

    fn submodule_file_id(&self, parent_id: FileId, ident: CustomIdentifier) -> ScopeResult<FileId>
    where
        Self: Sized,
    {
        let path = &*parent_id;

        should!(path_has_file_name(&path, "mod.hsk") || path_has_file_name(path, "main.hsk"));

        let module_path1 = path.with_file_name(format!("{}.hsk", ident.deref()));
        let module_path2 = path.with_file_name(format!("{}/mod.hsk", ident.deref()));

        let module_path = if module_path1.is_file() && !module_path2.is_file() {
            Ok(module_path1)
        } else if module_path2.is_file() && !module_path1.is_file() {
            Ok(module_path1)
        } else {
            Err(file::FileError::FileNotFound.into())
        };

        module_path.map(|pth| self.file_id(pth))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PackageOrModule {
    scope_id: ScopeId,
}

impl PackageOrModule {
    pub fn scope(&self) -> ScopeId {
        self.scope_id
    }
}