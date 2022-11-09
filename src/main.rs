#![allow(dead_code)]

use crate::writer::init;
use questions::var_write::AskUser;

mod questions;
mod input_reader;
mod writer;


pub const ASK_USER: AskUser = AskUser {
    open: "Answer by writing question like: Question1,Question2,Question3 or if you want to skip write n",
    drop_down: "Answer by writing question like: Question,answer1,answer2|Question2,answer1,answer2 or if you want to skip hit n",
};


fn main() {
    let ask = init();
    ask.open_question("pitOpenEnded", "Open Ended Pit Questions: ", true);
    ask.drop_question("pitDropDown", "DropDown Pit Questions: ");
    ask.open_question("matchOpenEnded", "Open Ended Match Scouting Questions: ", false);
    ask.drop_question("matchDropDown", "DropDown Scouting Questions: ");
}
