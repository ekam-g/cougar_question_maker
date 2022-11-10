use crate::input_reader::Input;
use crate::{ASK_USER, questions};

pub struct AskUser<'a> {
    pub open: &'a str,
    pub drop_down: &'a str,
}

impl Input {
    pub(crate) fn open_question(mut self, name : &str, question : &str) -> Self {
        let scouting1 = self.n_or_val(&format!("{}: {}", question, ASK_USER.open));
        if let Some(what) = scouting1 {
            let val = what.trim().split(',');
            questions::write_questions_firestore_opened(val, name, self.first_done);
            if !self.first_done {
                self.first_done = true;
            }
        }
        self
    }

    pub(crate) fn drop_question(self, name: &str, question : &str) -> Self {
        loop {
            let pit2 = self.n_or_val(&format!("{}: {}", question, ASK_USER.drop_down ));
            if let Some(what) = pit2 {
                let val = what.trim().split('|');
                match questions::write_questions_firestore_drop_down(val, name) {
                    Ok(_) => break,
                    Err(data) => println!("{}", data),
                }
            }
        }
        self
    }
    pub fn next_question(self) -> Self {
        self
    }
    pub fn new() -> Input {
        better_file_maker::make_folders("output").unwrap_or(());
        println!("output file created!");
        Input {
            first_done : false
        }
    }
}