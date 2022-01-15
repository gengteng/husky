use crate::*;

#[test]
fn haha() {
    let mut db = HuskyLangDatabase::new();
    let source: &'static str = r#"
dataset:
    synthetic::trivial::real1d::dataset1()

main:
    if input > 0.0:
        1
    else:
        0
"#;

    db.set_live_file_text("haha/main.hsk".into(), source.into());
    let main_file_id = db.file_id("haha/main.hsk".into());
    let ast_text = db.ast_text(main_file_id).unwrap();
    p!(ast_text.errors());
}