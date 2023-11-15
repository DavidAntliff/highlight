use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};

/// Command Line Argument Parser for Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The pattern to highlight
    pattern: String,

    /// Colour for highlighting (red, green, blue, yellow, cyan, magenta, white)
    #[clap(short, long, default_value = "red")]
    colour: String,

    /// Make the highlighting bold
    #[clap(short, long)]
    bold: bool,

    /// Files to search
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let regex = Regex::new(&args.pattern).expect("Invalid regex pattern");

    if args.files.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Could not read line");
            print_highlighted(&regex, &line, &args.colour, args.bold);
        }
    } else {
        for file in args.files {
            let content = fs::read_to_string(&file).expect("Could not read file");
            print_highlighted(&regex, &content, &args.colour, args.bold);
        }
    }
}

fn print_highlighted(regex: &Regex, text: &str, colour: &str, bold: bool) {

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
    let bold_code = if bold { "\x1b[1m" } else { "" };
    let highlighted_text = regex.replace_all(text, format!("{}{}$0\x1b[0m", bold_code, color_code));
    println!("{}", highlighted_text);
}
