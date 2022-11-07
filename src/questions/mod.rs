mod widgets;

use std::str::Split;



pub fn write_questions_firestore_opened(what: Split<char>) {
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in what {
            return_data = format!("'{}',{}", info, return_data);
        }
        return_data
    };
    crate::writer::try_write(format!("const List<String> firestoreLocation = [{}];", what_info), true);
}

pub fn write_widgets () {

}


