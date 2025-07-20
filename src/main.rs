use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

use md2html::MarkdownParser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        eprintln!("Usage: {} [input_file] [output_file]", args[0]);
        eprintln!("  If no files specified, reads from stdin and writes to stdout");
        eprintln!("  If only input_file specified, writes to stdout");
        process::exit(1);
    }

    let input = if args.len() > 1 {
        match fs::read_to_string(&args[1]) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading input file '{}': {}", args[1], e);
                process::exit(1);
            }
        }
    } else {
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!("Error reading from stdin: {e}");
            process::exit(1);
        }
        buffer
    };

    let mut parser = MarkdownParser::new(input);
    let html_output = parser.parse();

    if args.len() > 2 {
        if let Err(e) = fs::write(&args[2], html_output) {
            eprintln!("Error writing to output file '{}': {}", args[2], e);
            process::exit(1);
        }
        println!("Successfully converted '{}' to '{}'", args[1], args[2]);
    } else {
        print!("{html_output}");
    }
}