use std::fmt::Display;
use crate::input_reader::Input;

const LOCATION: &str = "output/questions.dart";

pub(crate) fn try_write<T: Display>(what: T, replace: bool) {
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

pub fn init() -> Input {
    better_file_maker::make_folders("output").unwrap_or(());
    println!("output file created!");
    Input {}
}