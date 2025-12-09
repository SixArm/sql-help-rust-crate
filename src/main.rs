//! sql-help
//!
//! Inspect your SQL code for improvements.
//!
//! This is a work in progress. Constructive feedback welcome.
//!
//! Inspired by:
//!
//! - https://github.com/ankane/strong_migrations
//!
//! - https://github.com/ayarotsky/diesel-guard
//!
//! Syntax:
//!
//! ```sh
//! sql-help <file>
//! cat <file> | sql-help
//! ```
//!
//! Examples:
//!
//! ```sh
//! sql-help my_file.sql>
//! cat my_file.sql | sql-help
//! ```

use std::fs;
use std::io::{self, Read};
use std::env;
mod scans;

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
    let output = match crate::scans::create_index_concurrently::scan(&sql) {
        Some((url, s)) => format!("{}\n\n{}", url, s),
        None => "".to_string(),
    };
    println!("{}", output);
    Ok(())
}
