#[cfg(test)]
mod tests;

pub use ast::{AstQueryGroup, AstSalsaQueryGroup};
pub use diagnostic::DiagnosticQuery;
pub use feature::{AllocateUniqueFeature, FeatureQueryGroup, FeatureQueryGroupStorage};
pub use file::{AllocateUniqueFile, FileQueryGroup, LiveFiles};
pub use husky_fmt::FmtQuery;
use scope::ScopePtr;
pub use scope::{AllocateUniqueScope, Scope};
pub use scope_query::{ScopeQueryGroup, ScopeSalsaQueryGroup};
pub use semantics::ControlEntityVersion;
use semantics::EntityKind;
pub use semantics::InferQueryGroup;
pub use semantics::SemanticQueryGroup;
pub use token::TokenQueryGroup;
pub use word::InternWord;

use common::*;

use std::fmt;

use stdx::sync::ARwLock;

#[salsa::database(
    file::FileQueryStorage,
    token::TokenQueryGroupStorage,
    scope_query::ScopeQueryGroupStorage,
    ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    semantics::SemanticQueryGroupStorage,
    semantics::MainQueryGroupStorage,
    semantics::EntityQueryGroupStorage,
    semantics::ConfigQueryGroupStorage,
    semantics::InferQueryGroupStorage,
    semantics::InstructionQueryGroupStorage,
    feature::FeatureQueryGroupStorage,
    diagnostic::DiagnosticQueryStorage,
    compiler::CompilerStorage
)]
pub struct HuskyLangCompileTime {
    storage: salsa::Storage<HuskyLangCompileTime>,
    file_unique_allocator: file::UniqueFileAllocator,
    word_unique_allocator: word::WordInterner,
    scope_unique_allocator: scope::UniqueScopeAllocator,
    live_docs: ARwLock<HashMap<file::FilePtr, ARwLock<String>>>,
    vc: semantics::EntityVersionControl,
    features: feature::FeatureUniqueAllocator,
}

impl fmt::Debug for HuskyLangCompileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HuskyLangDatabase").finish()
    }
}

impl salsa::Database for HuskyLangCompileTime {}

impl salsa::ParallelDatabase for HuskyLangCompileTime {
    fn snapshot(&self) -> salsa::Snapshot<HuskyLangCompileTime> {
        salsa::Snapshot::new(HuskyLangCompileTime {
            storage: self.storage.snapshot(),
            file_unique_allocator: self.file_unique_allocator.clone(),
            word_unique_allocator: self.word_unique_allocator.clone(),
            scope_unique_allocator: self.scope_unique_allocator.clone(),
            live_docs: self.live_docs.clone(),
            vc: self.vc.clone(),
            features: self.features.clone(),
        })
    }
}

impl Default for HuskyLangCompileTime {
    fn default() -> Self {
        Self {
            storage: Default::default(),
            file_unique_allocator: file::new_file_unique_allocator(),
            word_unique_allocator: word::new_word_unique_allocator(),
            scope_unique_allocator: scope::new_scope_unique_allocator(),
            live_docs: Default::default(),
            vc: semantics::EntityVersionControl::new(),
            features: feature::new_feature_unique_allocator(),
        }
    }
}

impl AllocateUniqueFile for HuskyLangCompileTime {
    fn file_unique_allocator(&self) -> &file::UniqueFileAllocator {
        &self.file_unique_allocator
    }
}

impl InternWord for HuskyLangCompileTime {
    fn word_unique_allocator(&self) -> &word::WordInterner {
        &self.word_unique_allocator
    }
}

impl LiveFiles for HuskyLangCompileTime {
    fn get_live_files(&self) -> &ARwLock<HashMap<file::FilePtr, ARwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: file::FilePtr) {
        file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyLangCompileTime {}

impl AllocateUniqueScope for HuskyLangCompileTime {
    fn scope_unique_allocator(&self) -> &scope::UniqueScopeAllocator {
        &self.scope_unique_allocator
    }
}

impl TokenQueryGroup for HuskyLangCompileTime {}

impl ScopeQueryGroup for HuskyLangCompileTime {}

impl AstQueryGroup for HuskyLangCompileTime {}

impl Upcast<dyn InferQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn semantics::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn SemanticQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn SemanticQueryGroup + 'static) {
        self
    }
}

impl ControlEntityVersion for HuskyLangCompileTime {
    fn entity_vc(&self) -> &vc::VersionControl<ScopePtr, EntityKind> {
        &self.vc
    }
}

impl AllocateUniqueFeature for HuskyLangCompileTime {
    fn features(&self) -> &feature::FeatureUniqueAllocator {
        &self.features
    }
}