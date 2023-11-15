use clap::{arg, command, Command};
use english_language_parser::*;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error reading or writing file `{0}`")]
    IOError(#[from] std::io::Error),
    #[error("Error parsing text: {0}")]
    ParseError(String),
    #[error("Error processing command line arguments: {0}")]
    ClapError(String),
}

fn main() -> Result<(), AppError> {
    let mut app = command!()
        .arg(
            arg!(-f --file <FILE> "Specify the file to parse")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output <FILE> "If set, saves the output to specified file")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .subcommand(Command::new("credits").about("Credits information"));

    let matches = app.clone().get_matches();

    if let Some(file_path) = matches.get_one::<PathBuf>("file") {
        let content = fs::read_to_string(file_path).map_err(AppError::IOError)?;
        let parsed_text = parse_text(&content)?;

        if let Some(output_file_path) = matches.get_one::<PathBuf>("output") {
            let formatted_text = parsed_text
                .iter()
                .map(|sentence| format!("{:?}", sentence))
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(output_file_path, formatted_text).map_err(AppError::IOError)?;
        } else {
            for sentence in parsed_text {
                println!("{:?}", sentence);
            }
        }
    } else if matches.subcommand_matches("credits").is_some() {
        display_credits();
    } else {
        app.print_long_help()
            .map_err(|e| AppError::ClapError(e.to_string()))?;
    }

    Ok(())
}

fn parse_text(content: &str) -> Result<Vec<Vec<&str>>, AppError> {
    english_parser::text(content).map_err(|_| AppError::ParseError(String::from("Parsing error")))
}

fn display_credits() {
    println!("Simple parser of English sentences created for KMA Rust course.");
    println!("By: Vadym Polishchuk");
}
