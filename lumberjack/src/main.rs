use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// I'm a lumberjack and I'm okay
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the log file to consume
    #[arg(short, long)]
    file: String,

    /// How to interpret the file
    #[arg(short, long, default_value_t = String::from("|"))]
    delimiter: String,

    /// Turn debugging information on
    #[arg(long, default_value_t = 4)]
    fields: u16,

    /// Turn debugging information on
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    if args.debug {
        println!("DEBUG ENABLED")
    }

    // Reading the file
    let file = File::open(args.file).expect("The log file could not be accessed");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let log_line = line.ok().unwrap();

        if args.debug {
            println!("DEBUG: LINE: {}", log_line);
        }

        let split_line: Vec<&str> = log_line.split(&args.delimiter).collect();

        if split_line.len() != args.fields.into() {}

        if args.debug {
            println!("DEBUG: START LINE SPLIT");
            for split in split_line {
                println!("{}", split);
            }
            println!("DEBUG: END LINE SPLIT");
        }
    }
}
