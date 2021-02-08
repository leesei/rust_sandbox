use std::env;

pub fn run() {
    let command = env::args().nth(2).expect("hello/status");
    let name = "Brad";
    let status = "100%";

    // println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
