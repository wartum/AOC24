use qmetaobject::prelude::*;
use cstr::cstr;
use std::collections::HashMap;

use crate::common::{get_input, Solution};
use crate::{day1, day2, day3, day4};

#[derive(QObject, Default)]
pub struct AOC24Solutions {
    // fields
    base: qt_base_class!(trait QObject),
    solutions: HashMap<i32, Solution>,
    inputs_dir: qt_property!(QString;),
    solution1: qt_property!(i32; NOTIFY solution1_changed),
    solution2: qt_property!(i32; NOTIFY solution2_changed),
    error_msg: qt_property!(QString; NOTIFY error_msg_changed),

    // signals
    solution1_changed: qt_signal!(),
    solution2_changed: qt_signal!(),
    error_msg_changed: qt_signal!(),

    // slots
    request_solution: qt_method!(
        fn request_solution(&mut self, day_number: i32) {
            self.request_solution_impl(day_number);
            self.solution1_changed();
            self.solution2_changed();
            self.error_msg_changed();
        }
    ),
}

impl AOC24Solutions {
    fn request_solution_impl(&mut self, day_number: i32) {
        match self.solutions.get(&day_number) {
            Some(solution) => {
                self.solution1 = solution.one_star_answer;
                self.solution2 = solution.two_star_answer;
                self.error_msg = QString::default();
            }
            None => match self.create_solution(day_number) {
                Err(msg) => self.error_msg = QString::from(msg),
                Ok(solution) => {
                    self.solutions.insert(day_number, solution);
                    self.request_solution_impl(day_number);
                }
            },
        }
    }

    fn create_solution(&mut self, day_number: i32) -> Result<Solution, String> {
        let input = get_input(day_number, &self.inputs_dir.to_string())?;
        match day_number {
            1 => day1::solve(input),
            2 => day2::solve(input),
            3 => day3::solve(input),
            4 => day4::solve(input),
            _ => Err(String::from("solution for given day not implemented")),
        }
    }
}

pub fn register_all_qml_types() {
    qml_register_type::<AOC24Solutions>(cstr!("AOCSolutions"), 1, 0, cstr!("AOCSolutions"));
}
