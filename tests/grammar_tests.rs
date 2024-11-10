use chat_commands_parcer::*;
use pest::Parser;
/*
#[test]
fn test_command() {
    let parsed = Grammar::parse(Rule::command, "/hello a b c").expect("Parce problem1");
    assert!(!parsed.as_str().is_empty());
}
*/
#[test]
fn test_user_command() {
    let parsed = Grammar::parse(Rule::user_command, "/smile").expect("Parce problem2");
    assert!(!parsed.as_str().is_empty());
}
#[test]
fn test_smile_command() {
    let parsed = Grammar::parse(Rule::smile_command, "/smile").expect("Parce problem2");
    assert!(!parsed.as_str().is_empty());
}
#[test]
fn test_repeat_command() {
    let parsed = Grammar::parse(Rule::user_command, "/repeat 3 \" hello\" ");
    assert!(parsed.is_err());
}
#[test]
fn test_define_command() {
    let parsed = Grammar::parse(Rule::define_command, "/define \"x\" \"value\"");
    assert!(parsed.is_err());
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
//Повинно бути порожнім так як ми використовуюмо "_"
#[test]
fn test_newline() {
    let parsed = Grammar::parse(Rule::NEWLINE, "\n").expect("Parce problem6");
    assert_eq!(parsed.as_str(), "");
}
//Повинно бути порожнім так як ми використовуюмо "_"

#[test]
fn test_quoted_text_with_quotes() {
    let input = "\"aaaaaa\"";
    let parsed = Grammar::parse(Rule::quoted_text, input).expect("Parce problem6");
    assert_eq!(parsed.as_str(), "\"aaaaaa\"");
}

#[test]
fn test_text() {
    let parsed = Grammar::parse(Rule::text, "dfgfsdfgsd").expect("Parce problem6");
    assert_eq!(parsed.as_str(), "dfgfsdfgsd");
}

#[test]
fn test_input() {
    let parsed = Grammar::parse(Rule::input, "dfgfsdfgsd").expect("Parce problem6");
    assert_eq!(parsed.as_str(), "dfgfsdfgsd");
}
