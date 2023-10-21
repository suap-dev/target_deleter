use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

fn main() {
    let mut paths: Vec<PathBuf> = Vec::new();

    dig("INVALID_PATH", &mut paths);

    for p in &paths {
        println!("{}", p.display());
    }

    for p in &paths {
        let _ = fs::remove_dir_all(p);
    }
}

fn dig<P: AsRef<Path>>(path: P, paths: &mut Vec<PathBuf>) {
    for folder in sub_folders(path) {
        if folder.file_name() == "target" {
            paths.push(folder.path().to_owned());
        } else {
            dig(folder.path(), paths);
        }
    }
}

fn sub_folders<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
    fs::read_dir(path)
        .expect("Invalid path.")
        .flatten()
        .filter(|entry| {
            // let is_dir = entry.file_type().unwrap().is_dir();
            // let is_read_only = entry.metadata().unwrap().permissions().readonly();
            // is_dir && !is_read_only
            entry.file_type().unwrap().is_dir()
        })
        .collect()
}
