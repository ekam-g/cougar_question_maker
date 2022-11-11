pub mod var_write;

use std::str::Split;


fn write_questions_firestore_opened(what: Split<char>, num_mode : bool) -> Vec<String> {
    let mut return_val = vec![];
    let what_info: String = {
        let mut return_data: String = "".to_owned();
        if num_mode {
            for info in what {
                return_data = format!("'{}':0,\n{}", info, return_data);
                return_val.push(format!("'{}'", info));
            }
        } else {
            for info in what {
                return_data = format!("'{}':'',\n{}", info, return_data);
                return_val.push(format!("'{}'", info));
            }
        }
        return_data
    };
    crate::writer::try_write(what_info, false);
    return_val
}

fn write_questions_firestore_drop_down(what: Split<char>, ) -> Result<(Vec<String> , Vec<String>), &str> {
    let header_vec_final: Vec<String>;
    let what_info = {
        let mut header_vec: Vec<String> = vec![];
        let mut return_data: Vec<String> = vec![];
        for info in what {
            let data = info.split(',');
            let mut section = "".to_owned();
            if data.clone().count() < 2 {
                return Err("Syntax Error, Please Make it Like this:  Question,answer1,answer2|Question2,answer1,answer2");
            }
            let mut first_done = false;
            for cool in data {
                if first_done {
                    section = format!("'{}',{}", cool, section);
                } else {
                    header_vec.push(cool.to_owned());
                    first_done = true;
                }
            }
            return_data.push(format!("[{}],", section));
        }
        header_vec_final = header_vec;
        return_data
    };
    for header_num in 0..header_vec_final.len() {
        crate::writer::try_write(
            format!("'{}':{}",
                    header_vec_final.get(header_num).unwrap_or(&"syntax error".to_owned()),
                    what_info.get(header_num).unwrap_or(&"syntax error".to_owned())
            ),
            false);
    }
    Ok((header_vec_final, what_info))
}

