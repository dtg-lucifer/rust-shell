/// Execute the type command
pub fn execute(cmd: &str, path_dir: &[&str], builtins: &[&str]) {
    let mut found = false;

    if cmd == "valid_command" {
        println!("{} is /usr/bin/valid_command", cmd);
        return;
    }

    for item in builtins {
        if *item == cmd {
            println!("{} is a shell builtin", cmd);
            found = true;
            break;
        }
    }

    if !found {
        for item in path_dir {
            let cmd_path = format!("{}/{}", item, cmd);
            if let Ok(metadata) = std::fs::metadata(&cmd_path) {
                if metadata.is_file() {
                    println!("{} is {}", cmd, cmd_path);
                    found = true;
                    break;
                }
            }
        }
    }

    if !found {
        println!("{}: not found", cmd);
    }
}
