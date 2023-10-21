use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() < 2 {
        ".".to_owned()
    } else {
        args[1].to_owned()
    };

    let mut paths: Vec<PathBuf> = Vec::new();
    dig(path, &mut paths);

    for p in &paths {
        println!("{}", p.display());
    }

    println!("Are you sure you want to delete all the above folders?\nTo confirm type [y]:");
    let mut user_response = String::new();
    let _ = std::io::stdin()
        .read_line(&mut user_response)
        .expect("Invalid input");
    user_response = user_response.trim().to_owned();

    if user_response == "y" || user_response == "Y" {
        println!("\nDeleting...");
        for p in &paths {
            let _ = fs::remove_dir_all(p);
        }
    } else {
        println!("\nExiting without deletion.");
    }
}

// The Rust community has developed guidelines for splitting the separate concerns of a binary program when main
// starts getting large. This process has the following steps:
// - Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
// - As long as your command line parsing logic is small, it can remain in main.rs.
// - When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
//
// The responsibilities that remain in the main function after this process should be limited to the following:
// - Calling the command line parsing logic with the argument values
// - Setting up any other configuration
// - Calling a run function in lib.rs
// - Handling the error if run returns an error
//
// This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic
// of the task at hand.

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
