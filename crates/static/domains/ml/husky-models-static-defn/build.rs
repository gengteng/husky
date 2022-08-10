use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
use husky_write_utils::w;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs::File};

static FILENAME: &str = &"husky_ml_models";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    write_rust_code("src/__rust_code_gen__.rs").unwrap();
}

pub static NONPRIMITIVE_TYPES: &'static [&'static str] = &["NaiveI32Internal"];

pub fn write_rust_code(rust_code_path: &str) -> std::io::Result<()> {
    let mut f = File::create(rust_code_path)
        .expect(&format!("rust code path {rust_code_path} doesn't exist"));
    w!(f; BuildCodeGenStart);
    w!(f; r#"
use crate::{*, naive::*};
use vm::*;

"#);
    for ty in NONPRIMITIVE_TYPES {
        w!(f; NonPrimitiveTypeRegistration { ty })
    }
    Ok(())
}