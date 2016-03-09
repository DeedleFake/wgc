mod words;

mod sort;

use std::env;
use std::collections::{BTreeSet};

fn matches(pat: &String, line: &str) -> bool {
    fn check<P, L>(mut pc: P, lc: L) -> bool where P: Iterator<Item=char>, L: Iterator<Item=char> {
        let mut c = pc.next();
        for outer in lc {
            match c {
                Some(ch) => {
                    if outer == ch {
                        c = pc.next();
                    }
                    else if words::consonant(outer) {
                        return false;
                    }
                },

                None => {
                    return true;
                }
            }
        }

        c.is_none()
    }

    (pat.len() <= line.len()) && (
        check(pat.chars(), line.chars()) ||
        check(pat.chars().rev(), line.chars().rev())
    )
}

fn usage(args: Vec<String>) -> ! {
    println!("Usage: {} <consonants>", args[0]);
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(args);
    }

    let cons = args[1].to_lowercase();
    if cons.chars().any(|c| {!words::consonant(c)}) {
        usage(args);
    }

    let words = words::WORDS.lines().map(|line| {
        line.to_lowercase()
    }).filter(|line| {
        matches(&cons, line)
    });

    let mut found = BTreeSet::new();
    for word in words {
        found.insert(sort::Length(word));
    }
    if found.len() == 0 {
        println!("No matches found.");
        return;
    }

    for word in found.iter().rev() {
        println!("{}", word.0);
    }
}
