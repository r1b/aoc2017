use std::io::{BufRead, Cursor};

pub fn count_valid(input: &'static str) -> u32 {
    let table: Vec<Vec<String>> = read_table(input);
    table.into_iter().map(|passphrase| if is_valid(passphrase) { 1 } else { 0 }).sum()
}

fn read_table(input: &'static str) -> Vec<Vec<String>> {
    let mut cursor = Cursor::new(input);
    let mut table: Vec<Vec<String>> = Vec::new();

    loop {
        let mut buf = String::new();
        let num_bytes = cursor.read_line(&mut buf).unwrap();

        if num_bytes == 0 {
            break;
        }
        else {
            table.push(buf.trim().split_whitespace().map(|word| word.to_string()).collect())
        }
    }

    table
}

fn is_valid(mut passphrase: Vec<String>) -> bool {
    let orig_len = passphrase.len();
    passphrase.sort();
    passphrase.dedup_by(|a, b| a == b);
    passphrase.len() == orig_len
}
