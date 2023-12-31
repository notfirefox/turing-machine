use std::{io::{self, BufRead}};

pub mod tm1;
pub mod turing;

use crate::tm1::accept;
use crate::tm1::delta;

fn parse_word(prompt: &str) -> Option<String> {
    println!("{}: ", prompt);
    let mut iterator = io::stdin().lock().lines();
    let line = match iterator.next()? {
        Ok(x) => x,
        Err(_) => return None,
    };
    Some(line.to_string())
}

fn main() {
    if let Some(word) = parse_word("Enter word") {
        let accept = Box::new(|x| accept(&x));
        let delta = Box::new(|x: &_| delta(&x));

        let mut tm = turing::TuringMachine::new(accept, delta);
        tm.process_word(&word);
    }
}
