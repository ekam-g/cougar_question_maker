use std::fmt::Display;
use txt_writer;

const LOCATION: &str = "output/questions.dart";


pub fn write_questions_firestore_opened(what: Vec<String>) {
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in &what {
            return_data = format!("'{}',{}", info, return_data);
        }
        return_data
    };
    try_write(format!("const List<String> firestoreLocation = [{}];", what_info));
}

pub fn make_open_questions(what: Vec<String>) -> String {
    let mut return_data = "".to_owned();
    for info in what {
        // txt_writer::WriteData {}.add(info, LOCATION).expect("failed when writing, please fix and try again");
        return_data = format!(r"openEndedQuestion(\n
        what : {},\n
        textCon : controler{}\n
        )", return_data, return_data);
    }
    return_data
}


fn try_write<T : Display>(what : T) {
    let mut error:i8 = 0;
    loop {
        let error_or_no = txt_writer::WriteData {}.replace(&what, LOCATION);
        if let Ok(_) = error_or_no {
            break;
        }else {
            error +=1;
            println!("error occurred when writing, Retrying");
            if error < 120 {
                panic!("please make sure we can write data to drive")
            }
        }

    }
}
struct widgets{}

impl widgets {

}