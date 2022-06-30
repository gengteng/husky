use std::path::PathBuf;

xflags::xflags! {
    cmd husky-debugger-command
        required dir: PathBuf
    {
        optional -v, --verbose
        optional --sample-id sample_id: String
        optional --mode mode: String
        optional -c, --compile
    }
}