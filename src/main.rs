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
                    println!("Info:");
                    for pair in parsed {
                        println!("{:?}", pair);
                    }
                }
                Err(e) => {
                    eprintln!("Failed: {}", e);
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
    println!("  parse <input_string> (parces string only if it is a type of command /example_identifier example arguments");
    println!("  help");
    println!("  credits\n");
}
