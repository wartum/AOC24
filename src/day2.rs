use crate::common::Solution;

pub fn solve(input: String) -> Result<Solution, String> {
    match parse_input(input) {
        Err(msg) => Err(msg),
        Ok(reports) => {
            let mut one_star_answer = 0;
            let mut two_star_answer = 0;

            for report in reports.iter() {
                if is_report_safe(&report) {
                    one_star_answer += 1;
                    two_star_answer += 1;
                } else {
                    for i in 0..report.len() {
                        let new_report = remove_from_report(report, i as i32);
                        if is_report_safe(&new_report) {
                            two_star_answer += 1;
                            break;
                        }
                    }
                }
            }

            return Ok(Solution {
                one_star_answer,
                two_star_answer,
            });
        }
    }
}

fn parse_input(input: String) -> Result<Vec<Vec<i32>>, String> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let levels: Result<Vec<i32>, _> = line.split_whitespace().map(|x| x.parse()).collect();
        match levels {
            Ok(lvl) => reports.push(lvl),
            Err(_) => return Err(String::from("Invalid report format")),
        }
    }
    return Ok(reports);
}

fn remove_from_report(report: &Vec<i32>, index: i32) -> Vec<i32> {
    let mut new_report = Vec::new();
    let mut i = -1;
    for level in report.iter() {
        i += 1;
        if i == index {
            continue;
        }
        new_report.push(*level);
    }
    return new_report;
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        eprintln!("report should contain at least 2 elements");
        return false;
    }
    let first_element = *report
        .get(0)
        .expect("report should contain at least 2 elements");
    let second_element = *report
        .get(1)
        .expect("report should contain at least 2 elements");

    let tendency_init = get_tendency(first_element, second_element);
    let mut previous_level = &first_element;

    for current_level in report.iter().skip(1) {
        let tendency_now = get_tendency(*previous_level, *current_level);
        if tendency_now != tendency_init {
            return false;
        }
        if (current_level - previous_level).abs() > 3 {
            return false;
        }
        previous_level = current_level;
    }
    return true;
}

fn get_tendency(a: i32, b: i32) -> i32 {
    if a == b {
        return 0;
    } else if a < b {
        return 1;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::day2;

    use super::remove_from_report;

    fn real_input() -> String {
        common::get_input(2, "resources/inputs").expect("Cannot get input")
    }

    fn sample_input() -> String {
        String::from(
            r#"7 6 4 2 1
               1 2 7 8 9
               9 7 6 2 1
               1 3 2 4 5
               8 6 4 4 1
               1 3 6 7 9"#,
        )
    }

    #[test]
    pub fn new_report() {
        let report = vec![1, 2, 3];
        let report_without_0 = remove_from_report(&report, 0);
        let report_without_1 = remove_from_report(&report, 1);
        let report_without_2 = remove_from_report(&report, 2);

        assert_eq!(report_without_0, vec![2, 3]);
        assert_eq!(report_without_1, vec![1, 3]);
        assert_eq!(report_without_2, vec![1, 2]);
    }

    #[test]
    pub fn sample_input_one_star_answer() {
        let solution = day2::solve(sample_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 2);
    }

    #[test]
    pub fn one_star_answer() {
        let solution = day2::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 220);
    }

    #[test]
    pub fn sample_input_two_star_answer() {
        let solution = day2::solve(sample_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 4);
    }

    #[test]
    pub fn two_star_answer() {
        let solution = day2::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 296);
    }
}
