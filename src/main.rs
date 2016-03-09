mod words;

mod sort;
use sort::{HeapPopDrain};

use std::env;
use std::collections::{BinaryHeap};

fn matches(pat: &String, line: &str) -> bool {
    fn check<P, L>(mut pc: P, lc: L) -> bool where P: Iterator<Item=char>, L: Iterator<Item=char> {
        let mut c = pc.next();
        for outer in lc {
            match c {
                Some(ch) => {
                    if outer == ch {
                        c = pc.next();
                        if c.is_none() {
                            return true;
                        }
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

        false
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
    if args[1].chars().any(|c| {!words::consonant(c)}) {
        usage(args);
    }

    let words = words::WORDS.lines().map(|line| {
        line.to_lowercase()
    }).filter(|line| {
        matches(&args[1], line)
    });

    let mut found = BinaryHeap::new();
    for word in words {
        found.push(sort::Length(word));
    }
    if found.len() == 0 {
        println!("No matches found.");
        return;
    }

    for word in found.pop_drain() {
        println!("{}", word.0);
    }
}
