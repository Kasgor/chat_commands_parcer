use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;
use std::env;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("No input enter some text/command to parce");
                return Ok(());
            }

            let input_string = args[2..].join(" ");
            match Grammar::parse(Rule::command, &input_string) {
                Ok(parsed) => {
                    println!("Parsed content:");
                    for pair in parsed {
                        println!("{:?}", pair);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse input: {}", e);
                }
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
    println!("  parse <input_string>");
    println!("  help");
    println!("  credits\n");
}
