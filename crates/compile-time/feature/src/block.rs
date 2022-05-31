mod func;
mod lazy;
mod proc;

pub use func::*;
pub use lazy::*;
pub use proc::*;

use file::FilePtr;
use semantics_eager::FuncStmt;
use semantics_entity::DefinitionRepr;
use semantics_lazy::*;
use std::sync::Arc;
use text::{TextRange, TextRanged};

use crate::{eval::FeatureEvalId, unique_allocate::FeatureUniqueAllocator, *};