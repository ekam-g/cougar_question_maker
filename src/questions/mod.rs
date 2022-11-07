use std::fmt::Display;
use std::str::Split;
use txt_writer;

const LOCATION: &str = "output/questions.dart";


pub fn write_questions_firestore_opened(what: Split<char>) {
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in what {
            return_data = format!("'{}',{}", info, return_data);
        }
        return_data
    };
    try_write(format!("const List<String> firestoreLocation = [{}];", what_info), true);
}


fn try_write<T: Display>(what: T, replace: bool) {
    let mut error: i8 = 0;
    loop {
        let error_or_no = {
            if replace {
                txt_writer::WriteData {}.replace(&what, LOCATION)
            } else {
                txt_writer::WriteData {}.add(&what, LOCATION)
            }
        };
        match error_or_no {
            Ok(_) => {
                break;
            }
            Err(data_error) => {
                error += 1;
                println!("error occurred when writing, Retrying\n {}", data_error);
                if error < 120 {
                    panic!("please make sure we can write data to drive\n{}", data_error)
                }
            }
        }
    }
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