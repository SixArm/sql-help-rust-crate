use std::fs;
use std::io::{self, Read};
use std::env;
mod help;

fn read_input() -> io::Result<String> {
    match env::args().nth(1) {
        Some(path) => fs::read_to_string(path),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn main() -> io::Result<()> {
    let sql = read_input()?;
    let output = match crate::help::create_index::help(&sql) {
        Some(s) => s,
        None => "".to_string(),
    };
    println!("{}", output);
    Ok(())
}
