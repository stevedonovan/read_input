//To run this example `cargo run --example yes_no --release`

extern crate dont_disappear;
extern crate read_input;

use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Choice(bool);

#[derive(Debug)]
struct ChoiceParseError {}

impl FromStr for Choice {
    type Err = ChoiceParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match char::from_str(s) {
            _ => Err(ChoiceParseError {}),
        }
    }
}

fn main() {
    println!("You inputted\n{:#?}", input_new::<Choice>().get());
    dont_disappear::enter_to_continue::default();
}
