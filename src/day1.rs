use std::collections::HashMap;

use crate::common::Solution;

pub fn solve(input: String) -> Result<Solution, String> {
    match parse_input(input) {
        Err(msg) => Err(msg),
        Ok(columns) => Ok(Solution {
            one_star_answer: calculate_distances(&columns),
            two_star_answer: calculate_similiarities(&columns),
        }),
    }
}

fn parse_input(input: String) -> Result<[Vec<i32>; 2], String> {
    let column1: Result<Vec<i32>, _> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .ok_or(String::from("first column doesn't exist"))?
                .parse::<i32>()
                .or(Err(String::from(
                    "cannot convert value from first column to integer",
                )))
        })
        .collect();

    let column2: Result<Vec<i32>, _> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .next()
                .ok_or(String::from("second column doesn't exist"))?
                .parse::<i32>()
                .or(Err(String::from("cannot convert value from second column to integer")))
        })
        .collect();

    match column1 {
        Err(msg) => Err(msg),
        Ok(mut col1) => match column2 {
            Err(msg) => Err(msg),
            Ok(mut col2) => {
                col1.sort();
                col2.sort();
                return Ok([col1, col2]);
            }
        }
    }
}

fn calculate_distances(columns: &[Vec<i32>; 2]) -> i32 {
    let mut distances = 0;
    let mut column1 = columns[0].iter();
    let mut column2 = columns[1].iter();

    for _ in 0..column1.len() {
        if let Some(v1) = column1.next() {
            if let Some(v2) = column2.next() {
                distances += (v1 - v2).abs();
            }
        }
    }

    return distances;
}

fn calculate_similiarities(columns: &[Vec<i32>; 2]) -> i32 {
    let mut dict = HashMap::new();
    for v in columns[0].iter() {
        let occurences_in_column2: i32 = columns[1].iter().filter(|x| **x == *v).count() as i32;
        dict.insert(v.to_owned(), v * occurences_in_column2);
    }

    return columns[0].iter().map(|v| dict.get(v).unwrap()).sum();
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::day1;

    fn real_input() -> String {
        common::get_input(1, "resources/inputs").unwrap()
    }

    fn sample_input() -> String {
        String::from(
            r#"3   4
               4   3
               2   5
               1   3
               3   9
               3   3"#,
        )
    }

    #[test]
    pub fn sample_input_one_star_answer() {
        let solution = day1::solve(sample_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 11);
    }

    #[test]
    pub fn sample_input_two_star_answer() {
        let solution = day1::solve(sample_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 31);
    }

    #[test]
    pub fn one_star_answer() {
        let solution = day1::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 3714264);
    }

    #[test]
    pub fn two_star_answer() {
        let solution = day1::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 18805872);
    }
}
