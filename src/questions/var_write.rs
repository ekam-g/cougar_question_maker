use crate::input_reader::Input;
use crate::{ASK_USER, questions};

pub struct AskUser<'a> {
    pub open: &'a str,
    pub drop_down: &'a str,
}


impl Input {
    pub(crate) fn open_question(&self, name : &str, question : &str, replace : bool) {
        let scouting1 = self.n_or_val(&format!("{}: {}", question, ASK_USER.open));
        if let Some(what) = scouting1 {
            let val = what.trim().split(',');
            questions::write_questions_firestore_opened(val, name, replace);
        }
    }

    pub(crate) fn drop_question(&self, name: &str, question : &str) {
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
    }
}