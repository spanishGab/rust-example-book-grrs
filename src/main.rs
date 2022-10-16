#![allow(unused)]

use clap::Parser;
use std::fmt;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cli<pattern='{}', path='{}'>", self.pattern, self.path.display())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Cli = Cli::parse();

    let file: File = File::open(args.path)
        .with_context(|| format!("Could not read file!"))?;

    let mut file_reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();

    loop {
        
        match file_reader.read_line(&mut line) {
            Ok(content) => {
                if content == 0 {
                    break;
                }
            },
            Err(error) => {
                return Err(error.into());
            }
        };
        
        if line.contains(&args.pattern) {
            println!("{}", line);
        }

        line.clear();
    }

    Ok(())
}
