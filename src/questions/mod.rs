mod widgets;

use std::str::Split;


pub fn write_questions_firestore_opened(what: Split<char>, name : &str ,replace : bool) {
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        for info in what {
            return_data = format!("'{}',{}", info, return_data);
        }
        return_data
    };
    crate::writer::try_write(format!("const List<String> {} = [{}];",name, what_info), replace);
}
//
// pub fn write_widgets(what: Split<char>) {
//     let wid = widgets::DartWidget {};
//     let write_info: String = {
//         let mut val = "\n".to_owned();
//         for make in what {
//             val = format!("{}\n{}", wid.make_text_place_holder(make), val)
//         }
//         val
//     };
//     crate::writer::try_write(format!("\n\n\n{}", write_info), false);
// }


