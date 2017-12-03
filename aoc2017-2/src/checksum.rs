use std::io::{BufRead, Cursor};

pub fn checksum(input: &'static str) -> u32 {
    let table: Vec<Vec<u32>> = read_table(input);
    let mut checksum: u32 = 0;

    for row in table {
        checksum += row.iter().max().unwrap() - row.iter().min().unwrap();
    }

    checksum
}

fn read_table(input: &'static str) -> Vec<Vec<u32>> {
    let mut cursor = Cursor::new(input);
    let mut table: Vec<Vec<u32>> = Vec::new();

    loop {
        let mut buf = String::new();
        let num_bytes = cursor.read_line(&mut buf).unwrap();

        if num_bytes == 0 {
            break;
        }
        else {
            table.push(buf.trim().split_whitespace().map(|item| u32::from_str_radix(item, 10).unwrap()).collect());
        }
    }

    table
}
