use husky_lang_debugger::*;
use loop::__init__::link_entity_with_compiled;
use compile_time_db::*;

#[tokio::main]
async fn main() {
    Debugger::new(|compile_time| init_compile_time(compile_time))
        .serve("localhost:51617")
        .await
        .expect("")
}

fn init_compile_time(compile_time: &mut HuskyLangCompileTime) {
    compile_time.load_pack("/home/xiyuzhai/Documents/husky/rust-gen/loop/snapshot".into());
    link_entity_with_compiled(compile_time)
}