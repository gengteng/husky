use crate::*;
use husky_root_static_defn::__resolve_root_defn;
use path_utils::collect_all_package_dirs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Mode {
    Run,
    Test,
}

// impl Mode {
//     pub async fn apply(
//         &self,
//         dir: &Path,
//         linkages_from_cdylib: &'static [(__StaticLinkageKey, __Linkage)],
//     ) {
//         match self {
//             Mode::Run => run(dir, linkages_from_cdylib).await,
//             Mode::Test => test_all_packages_in_dir(dir).await,
//         }
//     }
// }

impl From<Option<String>> for Mode {
    fn from(opt_str: Option<String>) -> Self {
        if let Some(ref s) = opt_str {
            match s.as_str() {
                "test" => Mode::Test,
                "run" => Mode::Run,
                _ => panic!(),
            }
        } else {
            Mode::Run
        }
    }
}

async fn run(package_dir: &Path, linkages_from_cdylib: &'static [(__StaticLinkageKey, __Linkage)]) {
    todo!()
    // HuskyDebugger::new_from_flags(linkages_from_cdylib)
    //     .serve("localhost:51617")
    //     .await
    //     .expect("")
}

// async fn test_all_packages_in_dir(dir: &Path) {
//     assert!(dir.is_dir());
//     let package_dirs = collect_all_package_dirs(dir);
//     println!(
//         "\n{}Running{} tests on {} example packages:",
//         husky_print_utils::CYAN,
//         husky_print_utils::RESET,
//         package_dirs.len()
//     );

//     for package_dir in package_dirs {
//         println!(
//             "\n{}test{} {}",
//             husky_print_utils::CYAN,
//             husky_print_utils::RESET,
//             package_dir.as_os_str().to_str().unwrap(),
//         );
//         let husky_debugger = HuskyDebuggerInstance::new(
//             HuskyDebuggerConfig {
//                 package_dir,
//                 opt_sample_id: Some(SampleId(23)),
//                 verbose: false,
//                 compiled: false,
//             },
//             &[],
//         );
//         match husky_debugger
//             .serve_on_error("localhost:51617", SampleId(0))
//             .await
//         {
//             TestResult::Success => finalize_success(),
//             TestResult::Failure => finalize_failure(),
//         }
//     }
// }