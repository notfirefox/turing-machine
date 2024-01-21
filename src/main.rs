use std::io::{self, BufRead};

pub mod tm1;
pub mod turing;

use crate::tm1::accept;
use crate::tm1::delta;

fn parse_word(prompt: &str) -> Option<String> {
    println!("{prompt}: ");
    io::stdin().lock().lines().next()?.ok()
}

fn main() {
    if let Some(word) = parse_word("Enter word") {
        let accept = Box::new(|x| accept(&x));
        let delta = Box::new(|x: &_| delta(x));

        let mut tm = turing::TuringMachine::new(accept, delta);
        tm.process_word(&word);
    }
}
