use anyhow::{anyhow, Result};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

/// Command Line Argument Parser for Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The pattern to highlight
    pattern: String,

    /// Colour for highlighting (red, green, blue, yellow, cyan, magenta, white)
    /// Can also specify a hex string, e.g. '0xff8020', to set a specific colour.
    /// Note that the bold flag has no effect with specific colours.
    #[clap(short, long, default_value = "red")]
    colour: String,

    /// Make the highlighting bold
    #[clap(short, long)]
    bold: bool,

    /// Files to search
    files: Vec<String>,
}

lazy_static! {
    static ref COLOUR_CODES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("red", "\x1b[31m");
        m.insert("green", "\x1b[32m");
        m.insert("blue", "\x1b[34m");
        m.insert("yellow", "\x1b[33m");
        m.insert("cyan", "\x1b[36m");
        m.insert("magenta", "\x1b[35m");
        m.insert("white", "\x1b[37m");
        m
    };
}

fn main() -> Result<()> {
    let args = Args::parse();

    let regex = Regex::new(&args.pattern)?;

    let code = get_format_code(&args.colour, args.bold)?;

    if args.files.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.expect("Could not read line");
            print_highlighted(&regex, &line, &code);
        }
    } else {
        for file in args.files {
            let content = fs::read_to_string(&file).expect("Could not read file");
            print_highlighted(&regex, &content, &code);
        }
    }

    Ok(())
}

fn get_format_code(colour: &str, bold: bool) -> Result<String> {
    let colour = colour.to_lowercase();
    let colour_code = match COLOUR_CODES.get(colour.as_str()) {
        Some(&code) => code,
        None => {
            if colour.starts_with("0x") {
                let (r, g, b) = hex_to_rgb(&colour)?;
                return Ok(format!("\x1b[38;2;{};{};{}m", r, g, b));
            }

            let available_colours = COLOUR_CODES.keys().cloned().collect::<Vec<_>>().join(", ");
            return Err(anyhow!(
                "Unexpected colour '{}'. Available colours: {}",
                colour,
                available_colours
            ));
        }
    };

    //let (r, g, b) = (255, 0, 255);
    //let colour_code = format!("\x1b[38;2;{};{};{}m", r, g, b);
    let bold_code = if bold { "\x1b[1m" } else { "" };
    Ok(format!("{}{}", bold_code, colour_code))
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8)> {
    // Validate the input format
    if !hex.starts_with("0x") {
        return Err(anyhow!("Invalid hex format"));
    }

    // Extract and parse the hexadecimal number
    let num =
        u32::from_str_radix(&hex[2..], 16).map_err(|_| anyhow!("Failed to parse hex number"))?;

    // Split the number into three 8-bit parts
    let r = ((num >> 16) & 0xFF) as u8;
    let g = ((num >> 8) & 0xFF) as u8;
    let b = (num & 0xFF) as u8;

    Ok((r, g, b))
}

fn print_highlighted(regex: &Regex, text: &str, code: &str) {
    let highlighted_text = regex.replace_all(text, format!("{}$0\x1b[0m", code));
    println!("{}", highlighted_text);
}
