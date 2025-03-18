use std::process::Command;

/// Execute a custom executable
pub fn execute(cmd: &str, args: &[&str], path_dir: &[&str]) -> bool {
    let mut found = false;

    for item in path_dir {
        let cmd_path = format!("{}/{}", item, cmd);
        if let Ok(metadata) = std::fs::metadata(&cmd_path) {
            if metadata.is_file() {
                Command::new(cmd)
                    .args(args)
                    .spawn()
                    .expect("Failed to execute command")
                    .wait()
                    .expect("Failed to wait for command");
                found = true;
                break;
            }
        }
    }

    if !found {
        println!("{}: not found", cmd);
    }

    true
}
