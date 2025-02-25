#[cfg(test)]
use crate::*;
#[cfg(test)]
#[test]
fn semantics_no_error_single_file() {
    let mut db = HuskyComptime::new_default(__resolve_root_defn);
    db.set_live_file_text(
        "haha/main.hsy".into(),
        r#"
struct A:
    a: i32

task:
    ml::datasets::synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = 1
    assert a == b
    a
"#
        .into(),
    );
    db.set_opt_target_entrance(Some(db.intern_file("haha/main.hsy".into())));
    let all_diagnostics = db.all_diagnostics();
    if all_diagnostics.len() > 0 {
        p!(all_diagnostics);
        panic!("diagnostic errors")
    }
    let main_file_id = db.intern_file("haha/main.hsy".into());
    let _pack = db.package(main_file_id).unwrap();
}
