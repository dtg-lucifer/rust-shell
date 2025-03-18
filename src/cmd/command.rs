use std::process::exit;

use super::{cd, custom_exe, echo, pwd, type_cmd};

/// List of built-in commands
pub fn get_builtins() -> Vec<&'static str> {
    vec!["echo", "type", "exit", "pwd"]
}

/// Main command handler function
pub fn handle_command(input: &str, path_dir: &[&str]) -> bool {
    match input {
        "exit" => exit(0),
        input if input.starts_with("custom_exe_") => {
            let parts: Vec<&str> = input.split_whitespace().collect();
            let cmd = parts[0];
            let args: Vec<&str> = parts.iter().skip(1).map(|s| *s).collect();
            custom_exe::execute(cmd, &args, path_dir)
        }
        input if input.starts_with("echo") => {
            echo::execute(&input[5..]);
            true
        }
        input if input.starts_with("type") => {
            type_cmd::execute(&input[5..], path_dir, &get_builtins());
            true
        }
        input if input == "pwd" => {
            pwd::execute();
            true
        }
        input if input.starts_with("cd") => {
            cd::execute(&input);
            true
        }
        _ => {
            println!("{}: command not found", input.trim());
            true
        }
    }
}
