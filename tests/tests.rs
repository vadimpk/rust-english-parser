extern crate english_language_parser;

use english_language_parser::english_parser;

#[test]
fn test_word() {
    assert_eq!(english_parser::word("hello"), Ok("hello"));
    assert_eq!(english_parser::word("hello-hello"), Ok("hello-hello"));
    assert_eq!(english_parser::word("hello's"), Ok("hello's"));
    assert_eq!(english_parser::word("hellos'"), Ok("hellos'"));
    assert_eq!(english_parser::word("hello-hello's"), Ok("hello-hello's"));
    assert!(english_parser::word("123").is_err());
    assert!(english_parser::word("hellos''").is_err());
}

#[test]
fn test_capital_word() {
    assert_eq!(english_parser::capital_word("Hello"), Ok("Hello"));
    assert_eq!(
        english_parser::capital_word("Hello-hello"),
        Ok("Hello-hello")
    );
    assert_eq!(english_parser::capital_word("Hello's"), Ok("Hello's"));
    assert_eq!(english_parser::capital_word("Hellos'"), Ok("Hellos'"));
    assert_eq!(
        english_parser::capital_word("Hello-hello's"),
        Ok("Hello-hello's")
    );
    assert!(english_parser::capital_word("123").is_err());
    assert!(english_parser::capital_word("hello'").is_err());
}

#[test]
fn test_number() {
    assert_eq!(english_parser::number("123"), Ok("123"));
    assert_eq!(english_parser::number("0"), Ok("0"));
    assert_eq!(english_parser::number("123.00"), Ok("123.00"));
    assert_eq!(english_parser::number("123.321312"), Ok("123.321312"));
    assert!(english_parser::number(".2321").is_err());
    assert!(english_parser::number("2312.").is_err());
    assert!(english_parser::number("word").is_err());
}

#[test]
fn test_date() {
    assert_eq!(english_parser::date("11/09/2001"), Ok("11/09/2001"));
    assert_eq!(english_parser::date("01/12/2023"), Ok("01/12/2023"));
    assert!(english_parser::date("01/12/789").is_err());
    assert!(english_parser::date("43/01/1000").is_err());
    assert!(english_parser::date("10/21/1000").is_err());
}

#[test]
fn test_hour() {
    assert_eq!(english_parser::hour("10:22 am"), Ok("10:22 am"));
    assert_eq!(english_parser::hour("01:59 pm"), Ok("01:59 pm"));
    assert_eq!(english_parser::hour("9:00 am"), Ok("9:00 am"));
    assert!(english_parser::hour("10:61 am").is_err());
    assert!(english_parser::hour("09:40").is_err());
    assert!(english_parser::hour("22:40 am").is_err());
}

#[test]
fn test_end_punctuation() {
    assert_eq!(english_parser::end_punctuation("!"), Ok("!"));
    assert_eq!(english_parser::end_punctuation("."), Ok("."));
    assert_eq!(english_parser::end_punctuation("?"), Ok("?"));
    assert_eq!(english_parser::end_punctuation("..."), Ok("..."));
    assert!(english_parser::end_punctuation("").is_err());
    assert!(english_parser::end_punctuation(",").is_err());
}

#[test]
fn test_other_punctuation() {
    assert_eq!(english_parser::other_punctuation(","), Ok(","));
    assert_eq!(english_parser::other_punctuation(":"), Ok(":"));
    assert_eq!(english_parser::other_punctuation(";"), Ok(";"));
    assert_eq!(english_parser::other_punctuation("-"), Ok("-"));
    assert!(english_parser::other_punctuation("").is_err());
    assert!(english_parser::other_punctuation(".").is_err());
}

#[test]
fn test_whitespace() {
    assert_eq!(english_parser::whitespace(" "), Ok(" "));
    assert_eq!(english_parser::whitespace("\n"), Ok("\n"));
    assert!(english_parser::whitespace("").is_err());
}

#[test]
fn test_sentence() {
    assert_eq!(
        english_parser::sentence("Hello, world!"),
        Ok(vec!["Hello", ",", " ", "world", "!"])
    );
    assert_eq!(
        english_parser::sentence("Hello, world 1234?"),
        Ok(vec!["Hello", ",", " ", "world", " ", "1234", "?"])
    );
    assert_eq!(
        english_parser::sentence("Hello: world; I am here."),
        Ok(vec![
            "Hello", ":", " ", "world", ";", " ", "I", " ", "am", " ", "here", "."
        ])
    );
    assert_eq!(
        english_parser::sentence("Hello, next-world..."),
        Ok(vec!["Hello", ",", " ", "next-world", "..."])
    );

    assert!(english_parser::sentence("not capital first letter.").is_err());
    assert!(english_parser::sentence("No ending punctuation").is_err());
}

#[test]
fn test_text() {
    assert_eq!(
        english_parser::text("Hello. Hello!"),
        Ok(vec![vec!["Hello", "."], vec!["Hello", "!"]])
    );
}
