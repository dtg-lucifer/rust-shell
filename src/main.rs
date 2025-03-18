mod cmd;

use std::env;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let env_var = env::var("PATH").unwrap_or_else(|_| "/usr/bin".to_string());
    let path_dir = env_var.split(':').collect::<Vec<&str>>();

    loop {
        print!("$ ");
        stdout.flush().unwrap();

        stdin.read_line(&mut input).unwrap();
        cmd::handle_command(input.trim(), &path_dir);
        input.clear();
    }
}
