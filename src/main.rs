mod words;

use std::env;

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

        false
    }

    check(pat.chars(), line.chars()) || check(pat.chars().rev(), line.chars().rev())
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

    let mut found = false;
    for word in words {
        println!("{}", word);
        found = true;
    }
    if !found {
        println!("No matches found.");
    }
}
