#![allow(dead_code)]

extern crate core;

use crate::input_reader::Input;

mod questions;
mod input_reader;
mod writer;

fn main() {
    let user = Input {};
    better_file_maker::make_folders("output").unwrap_or(());
    println!("output file created!");
    user.open_question("pitOpenEnded","Input Open Ended Questions for pit scouting and split them by comma if not write n" ,true);
    user.drop_question("pitDropDown","todo");
    user.open_question("matchOpenEnded","Input Open Ended Questions for match scouting and split them by comma if not write n", false);
}
