use crate::kalkulation::lagerkennzahlen;
use std::{collections::HashMap, io, io::Write};

pub fn start_repl() {
    println!("Welcome to the Rust CLI Calculation Helper!");
    let mut input = String::new();
    let command_map = get_commands();
    loop {
        print!("Command > ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let clean_input = clean_up_input(&input);
        let cmd = match command_map.get(&clean_input) {
            Some(c) => c,
            None => continue,
        };
        match cmd.command {
            Command::Exit => {
                println!("Closing CLI Tool!");
                println!("Goodbye!");
                break;
            }
            Command::Help => call_help(),
            Command::Umsatzhäufigkeit => break,
            Command::AvarageStoreTime => break,
        }
    }
}

fn call_help() {
    let cmds = get_commands();
    for (_, val) in cmds {
        println!("{}: {}", val.name, val.description);
    }
}
pub fn clean_up_input(input: &str) -> String {
    input.to_ascii_lowercase().trim().to_string()
}

enum Command {
    Help,
    Exit,
    Umsatzhäufigkeit,
    AvarageStoreTime,
}

struct CliCommand {
    name: String,
    description: String,
    command: Command,
}

fn get_commands() -> HashMap<String, CliCommand> {
    let mut command_map = HashMap::new();

    command_map.insert(
        String::from("help"),
        CliCommand {
            name: String::from("help"),
            description: String::from("Displays available commands"),
            command: Command::Help,
        },
    );
    command_map.insert(
        String::from("exit"),
        CliCommand {
            name: String::from("exit"),
            description: String::from("Closes CLI Tool"),
            command: Command::Exit,
        },
    );
    command_map.insert(
        String::from("um"),
        CliCommand {
            name: String::from("um"),
            description: String::from("Brechne Umsatzhäufigkeit"),
            command: Command::Umsatzhäufigkeit,
        },
    );
    command_map.insert(
        String::from("dld"),
        CliCommand {
            name: String::from("dld"),
            description: String::from("Berechne durchschnittliche Lagerdauer"),
            command: Command::AvarageStoreTime,
        },
    );

    command_map
}

fn get_values(parms: Vec<&str>) -> HashMap<String, f64> {
    let mut output = HashMap::new();
    let mut input = String::new();
    for s in parms {
        print!("{s}: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let val: f64 = input.trim().parse().unwrap_or(0.0);
        output.insert(String::from(s), val);
    }
    output
}
