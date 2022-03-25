use std::path::{Path, PathBuf};

pub fn path_has_file_name(path: &Path, name: &str) -> bool {
    path.file_name().map(|s| s.to_string_lossy()) == Some(name.into())
}

pub fn path_file_name_str(path: &Path) -> Option<String> {
    path.file_name().map(|s| s.to_string_lossy().to_string())
}

pub fn path_parent_file_name_str(path: &Path) -> Option<String> {
    if let Some(parent) = path.parent() {
        parent.file_name().map(|s| s.to_string_lossy().to_string())
    } else {
        None
    }
}

pub fn path_has_extension(path: &Path, extension: &str) -> bool {
    path.extension().map(|s| s.to_string_lossy()) == Some(extension.into())
}

pub fn collect_package_dirs(dir: PathBuf) -> Vec<PathBuf> {
    assert!(dir.is_dir());
    let main_path = dir.join("main.hsk");
    if main_path.exists() {
        return vec![dir];
    } else {
        let mut package_paths = vec![];
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let subpath = entry.path();
            if subpath.is_dir() {
                package_paths.extend(collect_package_dirs(subpath))
            }
        }
        package_paths
    }
}