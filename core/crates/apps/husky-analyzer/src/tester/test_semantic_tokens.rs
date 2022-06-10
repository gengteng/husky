use husky_compile_time::{utils::test_all_source_files, *};
use std::path::Path;
use test_utils::TestResult;
use token::AbsSemanticToken;

pub(super) fn test_semantic_tokens(package_dir: &Path) -> TestResult {
    test_all_source_files(package_dir, "semantic_tokens.txt", |compile_time, file| {
        match compile_time.ast_text(file) {
            Ok(ast_text) => AbsSemanticToken::to_semantic_tokens(&ast_text.semantic_tokens),
            Err(_) => Vec::new(),
        }
    })
}