use std::io::{self, BufRead, Write};

use lox_lang::scanner::Scanner;

pub fn run(code: &str) {
    let mut scanner = Scanner::new(code);

    println!("{:#?}", scanner.scan());
}

pub fn run_file(file_path: &str) {
    let code = std::fs::read_to_string(file_path).expect("Unable to read file");

    run(&code);
}

pub fn run_prompt() {
    println!("Entering prompt mode. Type 'q' to exit. \n\n");

    let mut line = String::new();
    // Avoid keep locking stdin/stdout, instead lock it only once
    let mut reader = io::stdin().lock();
    let mut writer = io::stdout().lock();

    loop {
        write!(writer, "> ").unwrap();
        writer.flush().unwrap();

        match reader.read_line(&mut line) {
            Ok(_) => {
                if line.trim() == "q" {
                    writeln!(writer, "Exiting prompt mode.").unwrap();
                    break;
                }

                run(&line);
            }
            Err(_) => {
                writeln!(writer, "Error reading line").unwrap();
                break;
            }
        }

        line.clear();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        n if n > 2 => {
            println!("No arguments provided.");
            std::process::exit(64);
        }
        2 => run_file(&args[1]),
        _ => run_prompt(),
    }

    println!("args: {:?}", args);
}
