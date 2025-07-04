use std::env;
use std::io::{self, Read};
use std::process;

use skimtoken::estimate_tokens_internal;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Help message
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        print_help();
        process::exit(0);
    }

    // Get text from args or stdin
    let text = if args.len() > 1 {
        // Join all arguments as the text
        args[1..].join(" ")
    } else if atty::is(atty::Stream::Stdin) {
        // No args and no piped input
        eprintln!("Error: No text provided");
        eprintln!();
        print_help();
        process::exit(1);
    } else {
        // Read from stdin
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => buffer.trim().to_string(),
            Err(e) => {
                eprintln!("Error reading from stdin: {e}");
                process::exit(1);
            }
        }
    };

    if text.is_empty() {
        eprintln!("Error: No text provided");
        process::exit(1);
    }

    // Estimate tokens and print result
    let token_count = estimate_tokens_internal(&text);
    println!("{token_count}");
}

fn print_help() {
    println!("Usage: skimtoken <text>");
    println!("       skimtoken --help");
    println!();
    println!("Calculate estimated token count for the given text.");
    println!();
    println!("Example:");
    println!("  skimtoken 'Hello, world!'");
    println!("  echo 'Some text' | skimtoken");
}
