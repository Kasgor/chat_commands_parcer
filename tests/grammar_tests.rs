use chat_commands_parcer::*;
use pest::Parser;

#[test]
fn test_command() {
    let parsed = Grammar::parse(Rule::command, "/hello a b c").expect("Parce problem1");
    assert!(!parsed.as_str().is_empty());
}

#[test]
fn test_user_command() {
    let parsed = Grammar::parse(Rule::user_command, "/hello a b c").expect("Parce problem2");
    assert!(!parsed.as_str().is_empty());
}

#[test]
fn test_identifier() {
    let parsed = Grammar::parse(Rule::identifier, "abc").expect("Parce problem3");
    assert_eq!(parsed.as_str(), "abc");
}

#[test]
fn test_argument() {
    let parsed = Grammar::parse(Rule::argument, "abc").expect("Parce problem4");
    assert_eq!(parsed.as_str(), "abc");
}

#[test]
fn test_whitespace() {
    let parsed = Grammar::parse(Rule::WHITESPACE, " ").expect("Parce problem5");
    assert_eq!(parsed.as_str(), "");
}

#[test]
fn test_newline() {
    let parsed = Grammar::parse(Rule::NEWLINE, "\n").expect("Parce problem6");
    assert_eq!(parsed.as_str(), "");
}
