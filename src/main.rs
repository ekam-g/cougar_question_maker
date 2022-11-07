#![allow(dead_code)]

extern crate core;

mod questions;
mod input_reader;
mod writer;

fn main() {
    let user = input_reader::Input {};
    better_file_maker::make_folders("output").unwrap_or(());
    println!("output file created!");
    let pit = user.n_or_val("Input Open Ended Questions for pit scouting and split them by comma if not write n");
    if let Some(what) = pit {
        let val = what.trim().split(',');
        questions::write_questions_firestore_opened(val, "pitOpenEnded", true);
    }
    loop {
        let pit2 = user.n_or_val("Input Open Ended Questions for pit scouting and split them by comma if not write n");
        if let Some(what) = pit2 {
            let val = what.trim().split(',');
            if questions::write_questions_firestore_drop_down(val, "pitDropDown").is_ok() {
                break;
            }
        }
    }
    let scouting1 = user.n_or_val("Input Open Ended Questions for pit scouting and split them by comma if not write n");
    if let Some(what) = scouting1 {
        let val = what.trim().split(',');
        questions::write_questions_firestore_opened(val, "scoutingOpenEnded", false);
    }

}
