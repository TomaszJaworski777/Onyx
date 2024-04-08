mod core;
mod eval;
mod mcts;
mod perft;
mod uci;

use std::io::stdin;
use uci::Uci;

fn main() {
    let mut uci = Uci::new();

    loop {
        let mut input_command = String::new();

        if stdin().read_line(&mut input_command).is_err() {
            println!("Error reading input, please try again.");
            continue;
        }

        let input_command = input_command.trim();

        let parts: Vec<&str> = input_command.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..].iter().map(|arg_str| arg_str.to_string()).collect::<Vec<String>>();

        if command == "exit" || command == "quit" {
            break;
        }

        uci.execute_command(command, args);
    }
}
