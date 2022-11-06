use txt_writer;

const LOCATION: &str = "output/questions.dart";


pub fn write_questions_firestore_opened(what: Vec<String>) {
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in what {
            return_data = format!("{},{}", info, return_data);
        }
        return_data
    };
    txt_writer::WriteData {}
        .replace(format!("List<String> firestoreLocation = [{}]", what_info), LOCATION)
        .expect("failed when writing, please fix and try again");
}

pub fn make_open_questions(what: Vec<String>) {
    const YES : &str = "";
    for info in what {
        txt_writer::WriteData {}.add(info, LOCATION).expect("failed when writing, please fix and try again");
    }
}
struct widgets{}

impl widgets {

}