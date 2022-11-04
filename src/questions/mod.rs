use txt_writer;

const LOCATION: &str = "output/";

pub fn make_text_box_widget() {}

pub fn write_questions_firestore(what: Vec<String>) {
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in what{
            return_data = format!("{},{}",info, return_data);
        }
      return_data
    };
    txt_writer::WriteData {}
        .replace(format!("List<String> firestoreLocation = [{}]", what_info),format!("{}{}", LOCATION, "questions.dart"))
        .expect("failed when writing, please fix and try again");
}
