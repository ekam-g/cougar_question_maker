#![allow(dead_code)]

use questions::var_write::AskUser;
use crate::input_reader::Input;

mod questions;
mod input_reader;
mod writer;


pub const ASK_USER: AskUser = AskUser {
    open: "Answer by writing question like: Question1,Question2,Question3 or if you want to skip write n",
    drop_down: "Answer by writing question like: Question,answer1,answer2|Question2,answer1,answer2 or if you want to skip hit n",
};


fn main() {
    Input::new()
        .open_question("Open Ended Questions: ", false, false)
        .next_question()
        .open_question("Number Questions: ", true , false)
        .next_question()
        .open_question("Number Questions(with arrow widget, so smaller numbers in 0-80 range): ", true , true)
        .next_question()
        .drop_question("DropDown Questions: ")
        .end()
}

//Todo Initial Data Map
//
// Map<String, dynamic> initialData = Map<String, dynamic>();
//
// this.initialData = const {
//         'Header': 'Match Scouting',
//         'Team Number': '',
//         'Match Number': '',
//         'Crossed the Tarmac': false,
//         'Autonomous Upper Hub': 0,
//         'Autonomous Lower Hub': 0,
//         'TeleOp Upper Hub': 0,
//         'TeleOp Lower Hub': 0,
//         'Endgame Upper Hub': 0,
//         'Endgame Lower Hub': 0,
//         'Starting Rung': "None",
//         'Ending Rung': "None",
//         'Upper Hub Missed': 0,
//         'Lower Hub Missed': 0,
//         'Comments': '',
//       }
//
//
//Todo Create a list of widgets
//
// List<Question>? matchFormQuestions;
//
// matchFormQuestions = [
//       ShortAnswer(
//         'Team Number',
//         TextInputType.number,
//         initialValue: widget.initialData['Team Number'],
//       ),
//       ShortAnswer('Match Number', TextInputType.number, initialValue: widget.initialData['Match Number']),
//       /* Autonomous */
//       CheckBoxQuestion(
//         'Crossed the Tarmac',
//         isChecked: widget.initialData['Crossed the Tarmac'],
//       ),
//       UpDownArrowQuestion(
//         'Autonomous Upper Hub',
//         counter: widget.initialData['Autonomous Upper Hub'],
//       ),
//       UpDownArrowQuestion(
//         'Autonomous Lower Hub',
//         counter: widget.initialData['Autonomous Lower Hub'],
//       ),
//       /* teleOp */
//       UpDownArrowQuestion(
//         'Upper Hub Missed',
//         counter: widget.initialData['Upper Hub Missed'],
//       ),
//       UpDownArrowQuestion(
//         'Lower Hub Missed',
//         counter: widget.initialData['Lower Hub Missed'],
//       ),
//       UpDownArrowQuestion(
//         'TeleOp Upper Hub',
//         counter: widget.initialData['TeleOp Upper Hub'],
//       ),
//       UpDownArrowQuestion(
//         'TeleOp Lower Hub',
//         counter: widget.initialData['TeleOp Lower Hub'],
//       ),
//       /*endgame*/
//       DropDownQuestion(
//         'Starting Rung',
//         ['None', 'Low', 'Middle'],
//         answer: widget.initialData['Starting Rung'],
//       ),
//       DropDownQuestion(
//         'Ending Rung',
//         ['None', 'Low', 'Middle', 'High', 'Traversal'],
//         answer: widget.initialData['Ending Rung'],
//       ),
//       ShortAnswer(
//         'Comments',
//         TextInputType.text,
//         initialValue: widget.initialData['Comments'],
//       ),
//     ];