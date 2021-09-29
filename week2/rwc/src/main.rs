use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut vec = vec![];
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        vec.push(line_str);
    }
    Ok(vec)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let lines = read_file_lines(filename).unwrap();
    let mut chars_len = 0;
    let mut bytes_len = 0;
    for line in lines.iter() {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        chars_len += words.len();
        bytes_len += line.len();
    }
    println!("{}  {}  {}  {}", lines.len(), chars_len, bytes_len+lines.len(), filename);
}
