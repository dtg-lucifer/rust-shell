use std::process::exit;

use super::{clear::clear, echo, ls};

// Main command handler function
pub fn handle_command(input: &str) -> bool {
  match input {
    "exit" => exit(0),
    "clear" => clear(),
    input if input.starts_with("ls") => {
        ls::ls(input);
        true
    }
    input if input.starts_with("echo") => {
      let args = input.split_whitespace().collect::<Vec<&str>>();
      echo::echo(args);
      true
    }
    _ => {
      println!("{}: command not found", input.trim());
      true
    }
  }
}
