extern crate core;

use better_file_maker;

mod questions;
mod input_reader;
mod writer;

fn main() {
    let user = input_reader::Input {};
    better_file_maker::make_folders("output").unwrap_or(());
    println!("output file created!");
    let pit = user.n_or_val("Input Open Ended Questions for pit scouting and split them by comma if not write n");
    if let Some(what) = pit {
        questions::write_questions_firestore_opened(what.trim().split(','));
    }
}
