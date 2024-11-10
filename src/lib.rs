use pest_derive::Parser;
use pest::Parser;

use std::collections::HashMap;
//use pest::{iterators::Pair, Parser};
// Існує проблема під час кліппі з PestError не вдалося вирішити
use thiserror::Error;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    #[error("Invalid number in repeat command")]
    InvalidNumber,
    #[error("Invalid expression in calc command: {0}")]
    InvalidExpression(String),
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    //#[error("Invalid condition in if command: {0}")]
    //InvalidCondition(String),
}

pub fn parse_input(input: &str) -> Result<String, ParseError> {
    let mut parsed = Grammar::parse(Rule::input, input)?;
    let mut output = String::new();
    let mut variables = HashMap::new();

    for pair in parsed.next().unwrap().into_inner() {
        
        for in_pair in pair.into_inner(){
            
            match in_pair.as_rule() {
                Rule::text | Rule::WHITESPACE => {
                    output.push_str(in_pair.as_str());
                }
                Rule::user_command => {
                    let command_pair = in_pair.into_inner().next().unwrap();
                    match command_pair.as_rule() {
                        Rule::smile_command => {
                            output.push_str(":) ");
                        }
                        Rule::repeat_command => {
                            let mut inner_rules = command_pair.into_inner();
                            let number_pair = inner_rules.next().unwrap();
                            let number: usize = number_pair.as_str().parse().map_err(|_| ParseError::InvalidNumber)?;
                            let text_pair = inner_rules.next().unwrap();
                            let quoted_text = text_pair.as_str();
                            let text_to_repeat = &quoted_text[1..quoted_text.len()-1];
                            for _ in 0..number {
                                output.push_str(text_to_repeat);
                            }
                        }
                        Rule::define_command => {
                            let mut inner_rules = command_pair.into_inner();
                            let var_name_pair = inner_rules.next().unwrap();
                            let var_value_pair = inner_rules.next().unwrap();
    
                            let var_name = &var_name_pair.as_str()[1..var_name_pair.as_str().len()-1];
                            let var_value = &var_value_pair.as_str()[1..var_value_pair.as_str().len()-1];
    
                            variables.insert(var_name.to_string(), var_value.to_string());
                        }
                        Rule::use_command => {
                            let var_name_pair = command_pair.into_inner().next().unwrap();
                            let var_name = &var_name_pair.as_str()[1..var_name_pair.as_str().len()-1];
    
                            if let Some(value) = variables.get(var_name) {
                                output.push_str(value);
                            } else {
                                return Err(ParseError::UndefinedVariable(var_name.to_string()));
                            }
                        }
                        _ => {
                            return Err(ParseError::UnknownCommand(command_pair.as_str().to_string()));
                        }
                    }
                }
                _ => {}
            }
        }
       
    }

    Ok(output)
}
