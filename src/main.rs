use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--tui" || a == "tui") {
        if let Err(e) = target_deleter::run_tui() {
            eprintln!("Error: {e}");
        }
    } else {
        let root_path = if args.len() < 2 { "." } else { &args[1] };
        target_deleter::run(root_path);
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
