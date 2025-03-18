use colored::Colorize;

/// Execute the echo command
pub fn echo(message: Vec<&str>) {
  for arg in message.iter().skip(1) {
    print!("{}", arg.blue().bold());

    if Some(&arg) != message.last().as_ref() {
      print!(" ");
    }
  }
  println!();
}
