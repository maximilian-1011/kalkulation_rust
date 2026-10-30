use crate::kalkulation::lagerkennzahlen;
use crate::kalkulation::verkaufskalkulation;
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
            Command::Umschlagshäufigkeit => {
                let parms: Vec<&str> = vec!["Umsatz", "Durchschnittliche Lagerbestand"];
                let val = get_values(parms);
                let res = lagerkennzahlen::umschlagshäufigkeit(
                    *val.get("Umsatz").unwrap(),
                    *val.get("Durchschnittliche Lagerbestand").unwrap(),
                );
                println!("\nDas Ergebnis ist: {res}\n");
                continue;
            }
            Command::AvarageStorage => {
                let parms: Vec<String> = get_bestände();
                let val = get_values(parms.iter().map(AsRef::as_ref).collect());
                let bestände: Vec<f64> = val.into_values().collect();
                let res = lagerkennzahlen::durchschnittlicher_lagerbestand(bestände);
                println!("\nDas Ergebnis ist: {res}\n");
                continue;
            }
            Command::AvarageStoreTime => {
                let parms: Vec<&str> = vec!["Umschlagshäufigkeit"];
                let val = get_values(parms);
                let res = lagerkennzahlen::durchschnittliche_lagerdauer(
                    *val.get("Umschlagshäufigkeit").unwrap(),
                );
                println!("\nDas Ergebnis ist: {res}\n");
                continue;
            }
            Command::BVP => {
                let parms: Vec<&str> = vec![
                    "Listenrpreis",
                    "Rabatt in %",
                    "Skonto in %",
                    "Bezugskosten",
                    "Handlunskosten in %",
                    "Gewinn in %",
                    "Umsatzsteuer in %",
                ];
                let val = get_values(parms);
                verkaufskalkulation::vollständigekalkulation(
                    *val.get("Listenrpreis").unwrap(),
                    *val.get("Rabatt in %").unwrap(),
                    *val.get("Skonto in %").unwrap(),
                    *val.get("Bezugskosten").unwrap(),
                    *val.get("Handlunskosten in %").unwrap(),
                    *val.get("Gewinn in %").unwrap(),
                    *val.get("Umsatzsteuer in %").unwrap(),
                );
                continue;
            }
            Command::LZS => {
                let parms: Vec<&str> = vec!["Jahreszinssatz", "Durchschnittliche Lagerdauer"];
                let val = get_values(parms);
                lagerkennzahlen::lagerzinssatz(
                    *val.get("Jahreszinssatz").unwrap(),
                    *val.get("Durchschnittliche Lagerdauer").unwrap(),
                );
                continue;
            }
            Command::LZ => {
                let parms: Vec<&str> = vec!["Lagerzinssatz", "Durchschnittlicher Lagerbestand"];
                let val = get_values(parms);
                lagerkennzahlen::lagerzins(
                    *val.get("Lagerzinssatz").unwrap(),
                    *val.get("Durchschnittlicher Lagerbestand").unwrap(),
                );
                continue;
            }
            Command::VV => {
                let parms: Vec<&str> = vec!["Kalkulationszuschlag in %", "Bezugspreis"];
                let val = get_values(parms);
                verkaufskalkulation::vereinfacht(
                    *val.get("Kalkulationszuschlag in %").unwrap(),
                    *val.get("Bezugspreis").unwrap(),
                );
                continue;
            }
            Command::KF => {
                let parms: Vec<&str> = vec!["Bezugspreis", "Bruttoverkaufspreis"];
                let val = get_values(parms);
                verkaufskalkulation::kalkulationsfaktor(
                    *val.get("Bezugspreis").unwrap(),
                    *val.get("Bruttoverkaufspreis").unwrap(),
                );
                continue;
            }
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
    Umschlagshäufigkeit,
    AvarageStorage,
    AvarageStoreTime,
    BVP,
    LZS,
    LZ,
    VV,
    KF,
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
            command: Command::Umschlagshäufigkeit,
        },
    );
    command_map.insert(
        String::from("dlb"),
        CliCommand {
            name: String::from("dlb"),
            description: String::from("Berechne durchschnittlichen Lagerbestand"),
            command: Command::AvarageStorage,
        },
    );
    command_map.insert(
        String::from("dld"),
        CliCommand {
            name: String::from("dld"),
            description: String::from("Berechne die durchschnittliche Lagerdauer"),
            command: Command::AvarageStoreTime,
        },
    );
    command_map.insert(
        String::from("bvp"),
        CliCommand {
            name: String::from("bvp"),
            description: String::from("Vollständige Bezugskalkulation"),
            command: Command::BVP,
        },
    );
    command_map.insert(
        String::from("lzs"),
        CliCommand {
            name: String::from("lzs"),
            description: String::from("Lagerzinssatz"),
            command: Command::LZS,
        },
    );
    command_map.insert(
        String::from("lz"),
        CliCommand {
            name: String::from("lz"),
            description: String::from("Lagerzinsen"),
            command: Command::LZ,
        },
    );
    command_map.insert(
        String::from("vv"),
        CliCommand {
            name: String::from("vv"),
            description: String::from("Verkürzte Vorwärtskalkulation"),
            command: Command::VV,
        },
    );
    command_map.insert(
        String::from("kf"),
        CliCommand {
            name: String::from("kf"),
            description: String::from("Kalkulationsfaktor"),
            command: Command::KF,
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
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let val: f64 = input.trim().parse().unwrap_or(0.0);
        output.insert(String::from(s), val);
    }
    output
}

fn get_bestände() -> Vec<String> {
    let mut input = String::new();
    loop {
        println!("Gib die Anzahl der Endbestände an");
        print!("| 1 | 4 | 12 | : ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let val: u8 = clean_up_input(&input).parse().unwrap();
        match val {
            1 => return vec!["Anfangsbestand".to_string(), "Endbestand".to_string()],
            4 => {
                return vec![
                    "Anfangsbestand".to_string(),
                    "1. Quartal".to_string(),
                    "2. Quartal".to_string(),
                    "3. Quartal".to_string(),
                    "4. Quartal".to_string(),
                ];
            }
            12 => {
                return vec![
                    "Anfangsbestand".to_string(),
                    "Jan".to_string(),
                    "Feb".to_string(),
                    "März".to_string(),
                    "April".to_string(),
                    "Mai".to_string(),
                    "Juni".to_string(),
                    "Juli".to_string(),
                    "Aug".to_string(),
                    "Sep".to_string(),
                    "Okt".to_string(),
                    "Nov".to_string(),
                    "Dez".to_string(),
                ];
            }
            other => {
                println!(
                    "{other} ist keine valide Anzahl. Bitte gib eine valide Anzahl an Endbeständen ein"
                );
                continue;
            }
        }
    }
}
