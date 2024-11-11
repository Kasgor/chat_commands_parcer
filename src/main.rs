use anyhow::Result;

use chat_commands_parcer::*;
use std::env;
use std::io::{self, Write};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            print!("Enter the name of the file: ");
            io::stdout().flush().unwrap();

            let mut filename = String::new();
            io::stdin()
                .read_line(&mut filename)
                .expect("Failed to read line");
            let filename = filename.trim();

            let input = match std::fs::read_to_string(filename) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", filename, e);
                    std::process::exit(1);
                }
            };

            match parse_input(&input) {
                Ok(output) => match std::fs::write("output.txt", output) {
                    Ok(_) => println!("Output written to output.txt"),
                    Err(e) => eprintln!("Error writing to output.txt: {}", e),
                },
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "help" => {
            print_help();
        }
        "credits" => {
            println!("Developed by: Kompaniiets Oleksandr");
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Help:\n");
    println!("Commands:");
    println!("  parse <input_file> (parces file");
    println!("  help");
    println!("  credits\n");
}
