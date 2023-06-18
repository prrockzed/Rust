use std::process::Command;

fn main() {
  // Elevate privilege to root
  let mut cmd = Command::new("sudo");
  cmd.arg("ls").arg("-la").arg("/root/");

  // Get the output of the command
  let output = cmd.output().expect("Failed to run command");

  // Print the output to the Flutter window
  println!("{}", String::from_utf8(output.stdout).unwrap());
}
