use crate::kalkulation::lagerkennzahlen;
use crate::kalkulation::verkaufskalkulation;
use std::{collections::HashMap, io, io::Write};

pub fn start_repl() {
    println!("Welcome to the Rust CLI Calculation Helper!");
    let mut input = String::new();
    let command_map = get_commands();
    loop {
        print!("Command > ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
            }
        };
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
            }
        };
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
                    match val.get("Umsatz") {
                        Some(val) => *val,
                        None => {
                            println!("Fehler beim lesen des Umsatzes");
                            continue;
                        }
                    },
                    match val.get("Durchschnittliche Lagerbestand") {
                        Some(val) => *val,
                        None => {
                            println!("Fehler beim lesen des Durchschnittlichen Lagerbestandes");
                            continue;
                        }
                    },
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
                    match val.get("Umschlagshäufigkeit") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                );
                println!("\nDas Ergebnis ist: {res}\n");
                continue;
            }
            Command::BruttoVerkaufsPreis => {
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
                    match val.get("Listenrpreis") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                    match val.get("Rabatt in %") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                    match val.get("Skonto in %") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                    match val.get("Bezugskosten") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                    match val.get("Handlunskosten in %") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                    match val.get("Gewinn in %") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                    match val.get("Umsatzsteuer in %") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten.");
                            continue;
                        }
                    },
                );
                continue;
            }
            Command::LagerZinsSatz => {
                let parms: Vec<&str> = vec!["Jahreszinssatz", "Durchschnittliche Lagerdauer"];
                let val = get_values(parms);
                lagerkennzahlen::lagerzinssatz(
                    match val.get("Jahreszinssatz") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
                    match val.get("Durchschnittliche Lagerdauer") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
                );
                continue;
            }
            Command::LagerZins => {
                let parms: Vec<&str> = vec!["Lagerzinssatz", "Durchschnittlicher Lagerbestand"];
                let val = get_values(parms);
                lagerkennzahlen::lagerzins(
                    match val.get("Lagerzinssatz") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
                    match val.get("Durchschnittlicher Lagerbestand") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
                );
                continue;
            }
            Command::VerküztesVerfahren => {
                let parms: Vec<&str> = vec!["Kalkulationszuschlag in %", "Bezugspreis"];
                let val = get_values(parms);
                verkaufskalkulation::vereinfacht(
                    match val.get("Kalkulationszuschlag in %") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
                    match val.get("Bezugspreis") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
                );
                continue;
            }
            Command::KalkulationsFaktor => {
                let parms: Vec<&str> = vec!["Bezugspreis", "Bruttoverkaufspreis"];
                let val = get_values(parms);
                verkaufskalkulation::kalkulationsfaktor(
                    match val.get("Bezugspreis") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
                    match val.get("Bruttoverkaufspreis") {
                        Some(val) => *val,
                        None => {
                            println!("Ein Unerwarteter Fehler ist aufgetreten");
                            continue;
                        }
                    },
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
    BruttoVerkaufsPreis,
    LagerZinsSatz,
    LagerZins,
    VerküztesVerfahren,
    KalkulationsFaktor,
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
            command: Command::BruttoVerkaufsPreis,
        },
    );
    command_map.insert(
        String::from("lzs"),
        CliCommand {
            name: String::from("lzs"),
            description: String::from("Lagerzinssatz"),
            command: Command::LagerZinsSatz,
        },
    );
    command_map.insert(
        String::from("lz"),
        CliCommand {
            name: String::from("lz"),
            description: String::from("Lagerzinsen"),
            command: Command::LagerZins,
        },
    );
    command_map.insert(
        String::from("vv"),
        CliCommand {
            name: String::from("vv"),
            description: String::from("Verkürzte Vorwärtskalkulation"),
            command: Command::VerküztesVerfahren,
        },
    );
    command_map.insert(
        String::from("kf"),
        CliCommand {
            name: String::from("kf"),
            description: String::from("Kalkulationsfaktor"),
            command: Command::KalkulationsFaktor,
        },
    );
    command_map
}

fn get_values(parms: Vec<&str>) -> HashMap<String, f64> {
    let mut output = HashMap::new();
    let mut input = String::new();
    for s in parms {
        print!("{s}: ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
            }
        };
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
            }
        };
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
        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
            }
        };
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
            }
        };
        let val: u8 = clean_up_input(&input).parse().unwrap_or(1);
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
