mod cmd;

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("Shell> ");
        stdout.flush().unwrap();

        stdin.read_line(&mut input).unwrap();
        if input.trim() == "" {
            continue;
        }
        cmd::handle_command(input.trim());
        input.clear();
    }
}
