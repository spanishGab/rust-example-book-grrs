#![allow(unused)]

use clap::Parser;
use std::fmt;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

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

fn main() {
    let args: Cli = Cli::parse();
    
    // println!("input args {}", args);

    let file: File = File::open(args.path).expect("could not read file");
    let mut file_reader: BufReader<File> = BufReader::new(file);

    let mut line: String = String::new();

    loop {
        line.clear();

        file_reader.read_line(&mut line);

        if line.contains(&args.pattern) {
            println!("{}", line);
        }

        if line.is_empty() {
            break
        };
    }


}
