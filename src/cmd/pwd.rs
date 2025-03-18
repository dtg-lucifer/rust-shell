use std::env;

/// Execute the pwd command
pub fn execute() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    } else {
        println!("Failed to get current directory");
    }
}
