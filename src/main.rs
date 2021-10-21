// Accept todo information as cli arguments
// Save the information in a file
// Read the information from a file and print it nicely
// Update information in the file
// Delete information
use std::env;
mod todo;
mod command;


fn main() {
    let args: Vec<String> = env::args().collect();
    let command = command::Command::from(args);
    println!("command:\n{:#?}", command);
}
