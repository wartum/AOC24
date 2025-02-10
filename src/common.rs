use std::error::Error;

pub fn get_input(day_number: i32, inputs_dir: &str) -> Result<String, Box<dyn Error>>  {
    let input_path = format!("{inputs_dir}/day{day_number}.txt");
    return match std::fs::read_to_string(&input_path) {
        Ok(input) => Ok(input),
        Err(err) => Err(format!("{}. \"{}\"", err.to_string(), input_path).into())
    }
}

pub struct Solution {
    pub one_star_answer: i32,
    pub two_star_answer: i32,
}