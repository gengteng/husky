mod flags;

use compile_time_db::*;
use compile_time_dir::{get_or_create_child_dir, get_rust_dir};
use file::FilePtr;
use io_utils::diff_write;
use path_utils::collect_package_dirs;
use print_utils::*;
use std::path::{Path, PathBuf};

pub fn compile_all(dir: PathBuf) {
    let package_dirs = collect_package_dirs(dir);
    for package_dir in package_dirs {
        compile_package(package_dir);
    }
}

pub fn compile_package(package_dir: PathBuf) {
    let mut compile_time = HuskyLangCompileTime::default();
    compile_time.load_package(package_dir.clone());
    let main_file = compile_time.unique_main_file();
    let package = compile_time.package(main_file).unwrap();
    let rust_dir = get_rust_dir(&package);
    let code_snapshot_dir = get_or_create_child_dir(&rust_dir, "snapshot");
    let src_dir = get_or_create_child_dir(&rust_dir, "src");
    let bin_dir = get_or_create_child_dir(&src_dir, "bin");

    save_code_snapshot(&compile_time, &code_snapshot_dir, main_file);

    // Cargo.toml
    diff_write(
        &rust_dir.join("Cargo.toml"),
        &compile_time.cargo_toml_content(main_file),
    );

    // lib.rs
    diff_write(
        &src_dir.join("lib.rs"),
        &compile_time.rust_lib_rs_content(main_file),
    );

    // bin/main.rs
    diff_write(
        &bin_dir.join("main.rs"),
        &compile_time.rust_bin_main_rs_content(main_file),
    );
}

fn save_code_snapshot(
    compile_time: &HuskyLangCompileTime,
    snapshot_dir: &Path,
    main_file: FilePtr,
) {
    diff_write(
        &snapshot_dir.join("main.hsk"),
        compile_time.file_content(main_file).to_str().unwrap(),
    );
    msg_once!("save snapshot of other files")
}