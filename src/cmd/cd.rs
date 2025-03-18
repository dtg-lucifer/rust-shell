pub fn execute(path: &str) -> bool {
    let path = &path[3..];
    let path = std::path::Path::new(path);
    if !path.exists() {
        println!("cd: {}: No such file or directory", path.display());
        return false;
    }
    let _ = std::env::set_current_dir(path);
    true
}
