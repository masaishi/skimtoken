use clap::Parser;
use skimtoken::estimate_tokens;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(version, about = "Estimate token count for text")]
struct Args {
    /// Text to estimate tokens for
    text: Option<String>,

    /// Read text from file
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let text = if let Some(file) = args.file {
        fs::read_to_string(file).unwrap_or_else(|e| {
            eprintln!("Error reading file: {e}");
            std::process::exit(1);
        })
    } else if let Some(text) = args.text {
        text
    } else if atty::is(atty::Stream::Stdin) {
        eprintln!("No text provided");
        std::process::exit(1);
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    if text.is_empty() {
        eprintln!("No text provided");
        std::process::exit(1);
    }

    let tokens = estimate_tokens(&text);
    println!("{tokens}");
}
