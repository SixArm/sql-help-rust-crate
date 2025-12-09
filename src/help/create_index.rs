//! Create index

use std::sync::LazyLock;
use regex::Regex;

/// Regex to match a SQL CREATE INDEX statement.
#[allow(dead_code)]
pub static REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
        r"(?x) # verbose mode
        (?<create_chunk>CREATE\s+)
        (?<unique_chunk>UNIQUE\s+)?
        (?<index_chunk>INDEX\s+)
        (?<if_not_exists_chunk>IF\s+NOT\s+EXISTS\s+)?
        (?<index_name_chunk>(?<index_name>\w+)\s+)
        (?<on_chunk>ON\s+)
        (?<table_chunk>(?<table_name>\w+)\s*)
        (?<column_chunk>\(\s*(?<column_names>[\w\s,]+)\s*\)\s*)
        ;
        "
    ).expect("REGEX")
});

/// Parse SQL and return help string.
#[allow(dead_code)]
pub fn help(sql: &str) -> Option<String> {
    match REGEX.captures(sql) {
        Some(captures) => {
            Some(format!("{}{}{}{}{}{}{}{}{}{}",
                &captures["create_chunk"],
                match captures.name("unique_chunk") { 
                    Some(x) => x.as_str(),
                    None => "",
                },
                &captures["index_chunk"],
                "CONCURRENTLY ",
                match captures.name("if_not_exists_chunk") { 
                    Some(x) => x.as_str(),
                    None => "",
                },
                &captures["index_name_chunk"],
                &captures["on_chunk"],
                &captures["table_chunk"],
                &captures["column_chunk"],
                ";"
            ))
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod regex {
        use super::*;
        
        #[test]
        fn simplest() {
            let sql = "CREATE INDEX my_index ON my_table (my_column) ;";
            let captures = REGEX.captures(sql).expect("captures");
            assert_eq!(&captures["create_chunk"], "CREATE ");
            assert!(captures.name("unique_chunk").is_none());
            assert_eq!(&captures["index_chunk"], "INDEX ");
            assert_eq!(&captures["index_name_chunk"], "my_index ");
            assert_eq!(&captures["index_name"], "my_index");
            assert!(captures.name("if_not_exists_chunk").is_none());
            assert_eq!(&captures["on_chunk"], "ON ");
            assert_eq!(&captures["table_chunk"], "my_table ");
            assert_eq!(&captures["table_name"], "my_table");
            assert_eq!(&captures["column_chunk"], "(my_column) ");
            assert_eq!(&captures["column_names"], "my_column");
        }

        #[test]
        fn with_whitespace() {
            let sql = "CREATE\nINDEX\nmy_index\nON\nmy_table\n(my_column)\n;";
            let captures = REGEX.captures(sql).expect("captures");
            assert_eq!(&captures["create_chunk"], "CREATE\n");
            assert!(captures.name("unique_chunk").is_none());
            assert_eq!(&captures["index_chunk"], "INDEX\n");
            assert_eq!(&captures["index_name_chunk"], "my_index\n");
            assert_eq!(&captures["index_name"], "my_index");
            assert!(captures.name("if_not_exists_chunk").is_none());
            assert_eq!(&captures["on_chunk"], "ON\n");
            assert_eq!(&captures["table_chunk"], "my_table\n");
            assert_eq!(&captures["table_name"], "my_table");
            assert_eq!(&captures["column_chunk"], "(my_column)\n");
            assert_eq!(&captures["column_names"], "my_column");
        }

        #[test]
        fn with_unique() {
            let sql = "CREATE UNIQUE INDEX my_index ON my_table (my_column) ;";
            let captures = REGEX.captures(sql).expect("captures");
            assert_eq!(&captures["create_chunk"], "CREATE ");
            assert_eq!(&captures["unique_chunk"], "UNIQUE ");
            assert!(captures.name("if_not_exists_chunk").is_none());
            assert_eq!(&captures["index_chunk"], "INDEX ");
            assert_eq!(&captures["index_name_chunk"], "my_index ");
            assert_eq!(&captures["index_name"], "my_index");
            assert!(captures.name("if_not_exists_chunk").is_none());
            assert_eq!(&captures["on_chunk"], "ON ");
            assert_eq!(&captures["table_chunk"], "my_table ");
            assert_eq!(&captures["table_name"], "my_table");
            assert_eq!(&captures["column_chunk"], "(my_column) ");
            assert_eq!(&captures["column_names"], "my_column");
        }

        #[test]
        fn with_if_not_exists() {
            let sql = "CREATE INDEX IF NOT EXISTS my_index ON my_table (my_column) ;";
            let captures = REGEX.captures(sql).expect("captures");
            assert_eq!(&captures["create_chunk"], "CREATE ");
            assert!(captures.name("unique_chunk").is_none());
            assert_eq!(&captures["if_not_exists_chunk"], "IF NOT EXISTS ");
            assert_eq!(&captures["index_chunk"], "INDEX ");
            assert_eq!(&captures["index_name_chunk"], "my_index ");
            assert_eq!(&captures["index_name"], "my_index");
            assert_eq!(&captures["if_not_exists_chunk"], "IF NOT EXISTS ");
            assert_eq!(&captures["on_chunk"], "ON ");
            assert_eq!(&captures["table_chunk"], "my_table ");
            assert_eq!(&captures["table_name"], "my_table");
            assert_eq!(&captures["column_chunk"], "(my_column) ");
            assert_eq!(&captures["column_names"], "my_column");
        }

        #[test]
        fn with_columns() {
            let sql = "CREATE INDEX my_index ON my_table (my_column_1, my_column_2, my_column_3) ;";
            let captures = REGEX.captures(sql).expect("captures");
            assert_eq!(&captures["create_chunk"], "CREATE ");
            assert!(captures.name("unique_chunk").is_none());
            assert_eq!(&captures["index_chunk"], "INDEX ");
            assert_eq!(&captures["index_name_chunk"], "my_index ");
            assert_eq!(&captures["index_name"], "my_index");
            assert!(captures.name("if_not_exists_chunk").is_none());
            assert_eq!(&captures["on_chunk"], "ON ");
            assert_eq!(&captures["table_chunk"], "my_table ");
            assert_eq!(&captures["table_name"], "my_table");
            assert_eq!(&captures["column_chunk"], "(my_column_1, my_column_2, my_column_3) ");
            assert_eq!(&captures["column_names"], "my_column_1, my_column_2, my_column_3");
        }

    }

    mod help {

        #[test]
        fn with_match() {
            let sql = "CREATE INDEX my_index ON my_table (my_column) ;";
            let s = super::help(sql).expect("help");
            assert_eq!(s, "CREATE INDEX CONCURRENTLY my_index ON my_table (my_column) ;");
        }

        #[test]
        fn without_match() {
            let sql = "";
            assert_eq!(super::help(sql), None)
        }

    }   

}
