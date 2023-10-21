use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

pub fn run(root_path: &str) {
    let mut target_folders: Vec<PathBuf> = Vec::new();
    find_target_paths(root_path, &mut target_folders);

    if confirm_deletion() {
        println!("\nDeleting...");
        delete_folders(&target_folders);
    } else {
        println!("\nExiting without deletion.");
    };
}

fn confirm_deletion() -> bool {
    println!("Do you want to delete all these folders?\nTo confirm type [y]:");
    let mut user_response = String::new();
    std::io::stdin()
        .read_line(&mut user_response)
        .expect("Invalid input");
    user_response = user_response.trim().to_owned();

    user_response == "y" || user_response == "Y"
}

fn delete_folders(target_folders: &Vec<PathBuf>) {
    for path in target_folders {
        fs::remove_dir_all(path).expect("Cannot remove folders");
    }
}

fn find_target_paths<P: AsRef<Path>>(path: P, paths: &mut Vec<PathBuf>) {
    for folder in sub_folders(path) {
        if folder.file_name() == "target" {
            let target_path = folder.path().to_owned();
            println!("{}", &target_path.display());
            paths.push(target_path);
        } else {
            find_target_paths(folder.path(), paths);
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
