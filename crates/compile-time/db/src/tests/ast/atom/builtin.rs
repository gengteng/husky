use super::utils;
use crate::*;
use atom::AtomKind;
use scope::{ScopeId, ScopeKind};
use word::{BuiltinIdentifier, Identifier};

#[test]
fn std_scope() {
    utils::check_atom_kind(
        "std",
        AtomKind::Scope(BuiltinIdentifier::Std.into(), ScopeKind::Module),
    );
}

#[test]
fn core_scope() {
    utils::check_atom_kind(
        "core",
        AtomKind::Scope(BuiltinIdentifier::Core.into(), ScopeKind::Module),
    );
}

#[test]
fn debug_scope() {
    utils::check_atom_kind(
        "debug",
        AtomKind::Scope(BuiltinIdentifier::Debug.into(), ScopeKind::Module),
    );
}

#[test]
fn i32_type() {
    utils::check_atom_kind(
        "i32",
        AtomKind::Scope(BuiltinIdentifier::I32.into(), ScopeKind::Type),
    );
}

#[test]
fn f32_type() {
    utils::check_atom_kind(
        "f32",
        AtomKind::Scope(BuiltinIdentifier::F32.into(), ScopeKind::Type),
    );
}

#[test]
fn vec_generics() {
    utils::check_atom_kind(
        "Vec",
        AtomKind::Scope(BuiltinIdentifier::Vector.into(), ScopeKind::Type),
    );
}

#[test]
fn tuple_generics() {
    utils::check_atom_kind(
        "Tuple",
        AtomKind::Scope(BuiltinIdentifier::Tuple.into(), ScopeKind::Type),
    );
}