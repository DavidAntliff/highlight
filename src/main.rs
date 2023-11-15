use clap::{App, Arg};
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let matches = App::new("highlight")
        .version("0.1")
        .about("Highlights patterns in text")
        .arg(
            Arg::new("pattern")
                .help("The pattern to highlight")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("files")
                .help("Files to search")
                .multiple_values(true)
                .index(2),
        )
        .arg(
            Arg::new("colour")
                .short('c')
                .long("colour")
                .takes_value(true)
                .help("Colour for highlighting (red, green, blue, yellow, cyan, magenta, white)"),
        )
        .arg(
            Arg::new("bold")
                .short('b')
                .long("bold")
                .help("Make the highlighting bold"),
        )
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let colour = matches.value_of("colour").unwrap_or("red");
    let bold = matches.is_present("bold");
    let regex = Regex::new(pattern).expect("Invalid regex pattern");

    match matches.values_of("files") {
        Some(files) => {
            for file in files {
                let content = fs::read_to_string(file).expect("Could not read file");
                print_highlighted(&regex, &content, colour, bold);
            }
        }
        None => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line.expect("Could not read line");
                print_highlighted(&regex, &line, colour, bold);
            }
        }
    }
}

fn print_highlighted(regex: &Regex, text: &str, colour: &str, bold: bool) {
    let bold_code = if bold { "\x1b[1m" } else { "" };

    let color_code = match colour {
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "blue" => "\x1b[34m",
        "yellow" => "\x1b[33m",
        "cyan" => "\x1b[36m",
        "magenta" => "\x1b[35m",
        "white" => "\x1b[37m",
        _ => "\x1b[0m",
    };

    //let (r, g, b) = (255, 0, 255);
    //let color_code = format!("\x1b[38;2;{};{};{}m", r, g, b);

    let highlighted_text = regex.replace_all(text, format!("{}{}$0\x1b[0m", bold_code, color_code));
    println!("{}", highlighted_text);
}
