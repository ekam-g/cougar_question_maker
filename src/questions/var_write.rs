use crate::input_reader::Input;
use crate::{ASK_USER, questions};
use crate::writer::try_write;

pub struct AskUser<'a> {
    pub open: &'a str,
    pub drop_down: &'a str,
}

impl Input {
    pub(crate) fn open_question(mut self, question: &str) -> Self {
        let scouting1 = self.n_or_val(&format!("{}: {}", question, ASK_USER.open));
        if let Some(what) = scouting1 {
            let val = what.trim().split(',');
            for add_val in questions::write_questions_firestore_opened(val) {
                self.question_vec.push(add_val);
            }
            if !self.first_done {
                self.first_done = true;
            }
        }
        self
    }

    pub(crate) fn drop_question(mut self, question: &str) -> Self {
        loop {
            let pit2 = self.n_or_val(&format!("{}: {}", question, ASK_USER.drop_down));
            if let Some(what) = pit2 {
                let val = what.trim().split('|');
                match questions::write_questions_firestore_drop_down(val) {
                    Ok((head, vals) ) => {
                        for header in head {

                        }
                        self.first_done = true;
                        break;
                    }
                    Err(data) => println!("{}", data),
                }
            } else {
                break;
            }
        }
        self
    }
    pub fn next_question(self) -> Self {
        try_write("//new question\n", false);
        self
    }
    pub fn new() -> Self {
        better_file_maker::make_folders("output").unwrap_or(());
        try_write("Map<String, dynamic> initialData = Map<String, dynamic>();\n\nthis.initialData = const {
        'Header': 'Match Scouting',
        'Team Number': 0,
        'Match Number': 0,", true);
        println!("output file created!");
        Self {
            first_done: true,
            question_vec : vec![],
            drop_down_header_vec : vec![],
            drop_down_val_vec : vec![],
        }
    }
    pub fn end(self) {
        try_write("};\nList<Question>? matchFormQuestions;\nmatchFormQuestions = [", false);
        try_write("", false);
        try_write("];", false);

    }
}