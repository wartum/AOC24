use qmetaobject::prelude::*;
use cstr::cstr;
use std::collections::HashMap;

use crate::common::{get_input, Solution};
use crate::{day1, day2, day3, day4};

#[derive(QObject, Default)]
pub struct AOC24Solutions {
    base: qt_base_class!(trait QObject),
    solutions: HashMap<i32, Solution>,
    inputs_dir: qt_property!(QString;),
    output: qt_property!(QString; NOTIFY output_changed),
    output_changed: qt_signal!(),
    request_solution: qt_method!(
        fn request_solution(&mut self, day_number: i32) {
            self.request_solution_impl(day_number);
            self.output_changed();
        }
    ),
}

impl AOC24Solutions {
    fn request_solution_impl(&mut self, day_number: i32) {
        match self.solutions.get(&day_number) {
            Some(solution) => {
                self.output = QString::from(format!(
                    "One Star: {}\nTwo stars: {}",
                    solution.one_star_answer, solution.two_star_answer
                ))
            }
            None => match self.create_solution(day_number) {
                Err(msg) => self.output = QString::from(msg),
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