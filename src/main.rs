#![allow(dead_code)]

extern crate core;

use crate::writer::init;

mod questions;
mod input_reader;
mod writer;

fn main() {
    let ask = init ();
    ask.open_question("pitOpenEnded", "Input Open Ended Questions for pit scouting and split them by comma if not write n", true);
    ask.drop_question("pitDropDown", "todo");
    ask.open_question("matchOpenEnded", "Input Open Ended Questions for match scouting and split them by comma if not write n", false);
}

