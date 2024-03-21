use std::io::{self, BufRead};

pub mod tm1;
pub mod tm2;
pub mod tm3;
pub mod tm4;
pub mod tm5;
pub mod tm6;
pub mod turing;

use crate::tm1::accept1;
use crate::tm1::delta1;

fn parse_word(prompt: &str) -> Option<String> {
    println!("{prompt}: ");
    io::stdin().lock().lines().next()?.ok()
}

fn main() {
    if let Some(word) = parse_word("Enter word") {
        let accept = Box::new(accept1);
        let delta = Box::new(delta1);

        let mut tm = turing::Machine::new(accept, delta);
        tm.process_word(&word);
    }
}
