use husky_bindgen_utils::simple_bindgen;
use husky_c_code_build::build_single_file_to_lib;
use husky_c_code_repr::*;
use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
use husky_write_utils::w;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs::File};

static FILENAME: &str = &"husky_ml_models";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let husky_dir = "/home/xiyuzhai/Documents/husky";
    let c_code_gen_dir = format!("{}/core/__c_code_gen__", husky_dir);
    gen_ml_models_code(&c_code_gen_dir);
    simple_bindgen(&c_code_gen_dir, FILENAME)
}

pub static NONPRIMITIVE_TYPES: &'static [&'static str] = &["NaiveI32Internal"];

pub fn gen_ml_models_code(c_code_gen_dir: &str) {
    let c_header_path = format!("{c_code_gen_dir}/{FILENAME}.h");
    let c_source_path = format!("{c_code_gen_dir}/{FILENAME}.c");
    // let husky_dir = std::env::var("HUSKY_DIR").expect("HUSKY_DIR is not set");
    let husky_dir = "/home/xiyuzhai/Documents/husky";
    let rust_code_path = format!(
        "{}/core/crates/static/domains/ml/husky-models-static-defn/src/__rust_code_gen__.rs",
        husky_dir
    );
    write_c_header(&c_header_path).unwrap();
    write_c_source(&c_source_path).unwrap();
    write_rust_code(&rust_code_path).unwrap();
    build_single_file_to_lib(&c_code_gen_dir, FILENAME);
}

pub fn write_c_header(c_header_path: &str) -> std::io::Result<()> {
    let mut buffer = File::create(c_header_path).unwrap();
    write!(
        buffer,
        r#"// this is auto generated
// do not modify by hand
#pragma once
#include "husky_vm_interface.h"
"#
    )?;
    for ty in NONPRIMITIVE_TYPES {
        write!(buffer, "{}", CNonPrimitiveTypeRegistrationHeader { ty })?
    }
    Ok(())
}

pub fn write_c_source(c_source_path: &str) -> std::io::Result<()> {
    use std::fmt::Display;

    let mut buffer = File::create(c_source_path).unwrap();
    write!(
        buffer,
        r#"#include "{FILENAME}.h"
    "#
    );

    for ty in NONPRIMITIVE_TYPES {
        write!(buffer, "{}", CNonPrimitiveTypeRegistrationSource { ty })?
    }
    Ok(())
}

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