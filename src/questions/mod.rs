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




struct widgets {}

impl widgets {
    pub fn make_open_questions(what: Vec<String>) -> String {
        let mut return_data = "".to_owned();
        for info in what {
            // txt_writer::WriteData {}.add(info, LOCATION).expect("failed when writing, please fix and try again");
            return_data = format!(r"openEndedQuestion(\n
        what : {},\n
        textCon : controler{}\n
        )", info, info);
        }
        return_data
    }
}