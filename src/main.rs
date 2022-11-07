use better_file_maker;
mod questions;
mod input_reader;

fn main() {
    better_file_maker::make_folders("output").unwrap_or(());
    println!("output file created!");
    println!("Input Open Ended Questions for pit scouting");
    questions::write_questions_firestore_opened(vec!["yes".to_owned(), "cool".to_owned()]);
    questions::write_questions_firestore_opened(vec!["yes".to_owned(), "cool".to_owned()]);

}
