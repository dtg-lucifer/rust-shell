/// Execute the echo command
pub fn echo(message: Vec<&str>) {
    for arg in &message {
        print!("{}", arg);

        if Some(&arg) != message.last().as_ref() {
            print!(" ");
        }
    }
    println!();
}
