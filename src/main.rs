use questions::var_write::AskUser;
use crate::input_reader::Input;

mod questions;
mod input_reader;
mod writer;


pub const ASK_USER: AskUser = AskUser {
    open: "Answer by writing question like: Question1,Question2,Question3 or if you want to skip write n",
    drop_down: "Answer by writing question like: Question,answer1,answer2|Question2,answer1,answer2 or if you want to skip hit n",
};


fn main() {
    Input::new()
        .open_question("pitOpenEnded", "Open Ended Pit Questions: ")
        .drop_question("pitDropDown", "DropDown Pit Questions: ")
        .open_question("matchOpenEnded", "Open Ended Match Scouting Questions: ")
        .drop_question("matchDropDown", "DropDown Scouting Questions: ");
}
