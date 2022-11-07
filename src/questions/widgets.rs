// pub struct DartWidget {}
//
// impl DartWidget {
//     pub fn make_open_questions(what: Vec<String>) -> String {
//         let mut return_data = "".to_owned();
//         for info in what {
//             // txt_writer::WriteData {}.add(info, LOCATION).expect("failed when writing, please fix and try again");
//             return_data = format!(r"openEndedQuestion(\n
//             what : {},\n
//             textCon : controler{}\n
//         )", info, info);
//         }
//         return_data
//     }
//     pub fn make_text_place_holder(&self, input : &str) -> String {
//         return format!("Text('{}'),", input);
//     }
// }