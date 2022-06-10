mod generics;
mod lambda;
mod root;
mod utils;

use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyCompileTime::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

main:
    let a = 1
"#
        .into(),
    );

    let main_file_id = db.intern_file("haha/main.hsk".into());
    let ast_main_file = db.ast_text(main_file_id);
    p!(ast_main_file);
}