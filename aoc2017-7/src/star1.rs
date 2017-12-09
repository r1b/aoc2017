use std::collections::HashSet;
use std::io::{BufRead, Cursor};
use regex::Regex;

// We can reduce the problem to: what is the name of a non-leaf node
// on the lhs that does not appear in any parents on the lhs

pub fn bottom(input: &'static str) -> String {
    let (nodes, parents) = parse_tree(input);

    for node in nodes {
        if !parents.contains(&node) {
            return node;
        }
    }

    String::from("")
}

fn parse_tree(input: &'static str) -> (HashSet<String>, HashSet<String>) {
    let mut cursor = Cursor::new(input);
    let mut nodes = HashSet::new();
    let mut parents = HashSet::new();
    let re = Regex::new(r"([a-z]+) \((\d+)\) -> ([a-z, ]*)").unwrap();

    loop {
        let mut buf = String::new();
        let num_bytes = cursor.read_line(&mut buf).unwrap();

        if num_bytes == 0 {
            break;
        }
        else {
            match re.captures(&buf) {
                None        => { continue; },
                Some(cap)   => {
                    nodes.insert(String::from(cap.get(1).unwrap().as_str()));
                    for parent in cap.get(3).unwrap().as_str().split(", ") {
                        parents.insert(String::from(parent));
                    }
                }
            }
        }
    }

    (nodes, parents)
}
