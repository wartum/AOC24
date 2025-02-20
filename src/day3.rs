use crate::common::Solution;

enum MulState {
    Clear,
    M,
    U,
    L,
    OpenBracket,
    Number1,
    Comma,
    Number2,
    CloseBracket,
}

enum DoState {
    Clear,
    D,
    O,
    OpenBracket,
    CloseBracket,
}

enum DontState {
    Clear,
    D,
    O,
    N,
    Tick,
    T,
    OpenBracket,
    CloseBracket,
}

struct LexicalAnalyzer {
    mul_state: MulState,
    do_state: DoState,
    dont_state: DontState,
    buffer: String,
    output: String,
}

impl LexicalAnalyzer {
    fn new() -> Self {
        Self {
            mul_state: MulState::Clear,
            do_state: DoState::Clear,
            dont_state: DontState::Clear,
            buffer: String::with_capacity(10),
            output: String::with_capacity(100),
        }
    }

    fn put_char(&mut self, c: char) {
        self.set_mul_state(c);
        self.set_do_state(c);
        self.set_dont_state(c);
    }

    fn finalize(&mut self) {
        self.put_char(' ');
    }

    fn set_mul_state(&mut self, c: char) {
        match self.mul_state {
            MulState::Clear => {
                if c == 'm' {
                    self.progress_mul_state(MulState::M, c);
                } else {
                    self.progress_mul_state(MulState::Clear, c);
                }
            }
            MulState::M => {
                if c == 'u' {
                    self.progress_mul_state(MulState::U, c);
                } else {
                    self.progress_mul_state(MulState::Clear, c);
                }
            }
            MulState::U => {
                if c == 'l' {
                    self.progress_mul_state(MulState::L, c);
                } else {
                    self.progress_mul_state(MulState::Clear, c);
                }
            }
            MulState::L => {
                if c == '(' {
                    self.progress_mul_state(MulState::OpenBracket, c);
                } else {
                    self.progress_mul_state(MulState::Clear, c);
                }
            }
            MulState::OpenBracket => {
                if c.is_ascii_digit() {
                    self.progress_mul_state(MulState::Number1, c)
                } else {
                    self.progress_mul_state(MulState::Clear, c);
                }
            }
            MulState::Number1 => {
                if c.is_ascii_digit() {
                    self.progress_mul_state(MulState::Number1, c)
                } else if c == ',' {
                    self.progress_mul_state(MulState::Comma, c)
                } else {
                    self.progress_mul_state(MulState::Clear, c)
                }
            }
            MulState::Comma => {
                if c.is_ascii_digit() {
                    self.progress_mul_state(MulState::Number2, c)
                } else {
                    self.progress_mul_state(MulState::Clear, c);
                }
            }
            MulState::Number2 => {
                if c.is_ascii_digit() {
                    self.progress_mul_state(MulState::Number2, c)
                } else if c == ')' {
                    self.progress_mul_state(MulState::CloseBracket, c)
                } else {
                    self.progress_mul_state(MulState::Clear, c)
                }
            }
            MulState::CloseBracket => {
                if self.output.len() > 0 {
                    self.output.push(' ');
                }
                self.output.push_str(&self.buffer);

                self.buffer.clear();
                self.mul_state = MulState::Clear;
                self.set_mul_state(c);
            }
        }
    }

    fn set_do_state(&mut self, c: char) {
        match self.do_state {
            DoState::Clear => {
                if c == 'd' {
                    self.do_state = DoState::D
                }
            }
            DoState::D => {
                if c == 'o' {
                    self.do_state = DoState::O
                } else {
                    self.do_state = DoState::Clear
                }
            }
            DoState::O => {
                if c == '(' {
                    self.do_state = DoState::OpenBracket
                } else {
                    self.do_state = DoState::Clear
                }
            }
            DoState::OpenBracket => {
                if c == ')' {
                    self.do_state = DoState::CloseBracket
                } else {
                    self.do_state = DoState::Clear
                }
            }
            DoState::CloseBracket => {
                if self.output.len() > 0 {
                    self.output.push(' ');
                }
                self.output.push_str("do()");
                self.do_state = DoState::Clear;
                self.set_do_state(c);
            }
        }
    }

    fn set_dont_state(&mut self, c: char) {
        match self.dont_state {
            DontState::Clear => {
                if c == 'd' {
                    self.dont_state = DontState::D
                }
            }
            DontState::D => {
                if c == 'o' {
                    self.dont_state = DontState::O
                } else {
                    self.dont_state = DontState::Clear
                }
            }
            DontState::O => {
                if c == 'n' {
                    self.dont_state = DontState::N
                } else {
                    self.dont_state = DontState::Clear
                }
            }
            DontState::N => {
                if c == '\'' {
                    self.dont_state = DontState::Tick
                } else {
                    self.dont_state = DontState::Clear
                }
            }
            DontState::Tick => {
                if c == 't' {
                    self.dont_state = DontState::T
                } else {
                    self.dont_state = DontState::Clear
                }
            }
            DontState::T => {
                if c == '(' {
                    self.dont_state = DontState::OpenBracket
                } else {
                    self.dont_state = DontState::Clear
                }
            }
            DontState::OpenBracket => {
                if c == ')' {
                    self.dont_state = DontState::CloseBracket
                } else {
                    self.dont_state = DontState::Clear
                }
            }
            DontState::CloseBracket => {
                if self.output.len() > 0 {
                    self.output.push(' ');
                }
                self.output.push_str("don't()");
                self.dont_state = DontState::Clear;
                self.set_dont_state(c);
            }
        }
    }

    fn progress_mul_state(&mut self, state: MulState, c: char) {
        if let MulState::Clear = state {
            self.buffer.clear();
        } else {
            self.buffer.push(c);
        }
        self.mul_state = state;
    }
}

pub fn solve(input: String) -> Result<Solution, String> {
    let mut lexer = LexicalAnalyzer::new();
    for c in input.chars() {
        lexer.put_char(c);
    }
    lexer.finalize();

    let parsed = lexer.output;
    let tokens_basic = tokenize(&parsed, false);
    let tokens_advanced = tokenize(&parsed, true);

    Ok(Solution {
        one_star_answer: calculate(&tokens_basic),
        two_star_answer: calculate(&tokens_advanced),
    })
}

fn calculate(tokens: &Vec<(i32, i32)>) -> i32 {
    tokens.iter().map(|pair| pair.0 * pair.1).sum()
}

fn tokenize(input: &str, toggle_support: bool) -> Vec<(i32, i32)> {
    let mut tokens = Vec::new();
    let mut do_flag = true;

    for word in input.split_whitespace() {
        if toggle_support {
            if word == "do()" {
                do_flag = true;
            } else if word == "don't()" {
                do_flag = false;
            }
        }

        if do_flag && word.starts_with("mul(") {
            tokens.push(extract_numbers(word).unwrap_or_default());
        }
    }

    return tokens;
}

fn extract_numbers(input: &str) -> Option<(i32, i32)> {
    let mut splits = input.split(',');
    let split1 = splits.next()?;
    let split2 = splits.next()?;
    let num1: i32 = split1
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()?;
    let num2: i32 = split2
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()?;
    return Some((num1, num2));
}

#[cfg(test)]
mod tests {
    use crate::{common, day3};

    use super::{tokenize, LexicalAnalyzer};

    fn sample_input() -> String {
        return String::from(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
    }

    fn sample_input_2() -> String {
        return String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
    }

    fn real_input() -> String {
        common::get_input(3, "resources/inputs").expect("Cannot get input")
    }

    #[test]
    pub fn lexical_analysis_basic() {
        let input = sample_input();
        let mut lexer = LexicalAnalyzer::new();

        for c in input.chars() {
            lexer.put_char(c);
        }
        lexer.finalize();

        assert_eq!(lexer.output, "mul(2,4) mul(5,5) mul(11,8) mul(8,5)");
    }

    #[test]
    pub fn lexical_analysis_advanced() {
        let input = sample_input_2();
        let mut lexer = LexicalAnalyzer::new();

        for c in input.chars() {
            lexer.put_char(c);
        }
        lexer.finalize();

        assert_eq!(
            lexer.output,
            "mul(2,4) don't() mul(5,5) mul(11,8) do() mul(8,5)"
        );
    }

    #[test]
    pub fn tokenization() {
        let input_basic = String::from("mul(2,4) mul(5,5) mul(11,8) mul(8,5)");
        let input_advanced = String::from("mul(2,4) don't() mul(5,5) mul(11,8) do() mul(8,5)");

        let tokens_basic = tokenize(&input_basic, false);
        let tokens_adv_wo_toggle = tokenize(&input_advanced, false);
        let tokens_adv_w_toggle = tokenize(&input_advanced, true);

        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5)], tokens_basic);
        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5)], tokens_adv_wo_toggle);
        assert_eq!(vec![(2, 4), (8, 5)], tokens_adv_w_toggle);
    }

    #[test]
    pub fn sample_input_one_star_answer() {
        let solution = day3::solve(sample_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 161);
    }

    #[test]
    pub fn one_star_answer() {
        let solution = day3::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 189600467);
    }

    #[test]
    pub fn sample_input_two_star_answer() {
        let solution = day3::solve(sample_input_2()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 48);
    }

    #[test]
    pub fn two_star_answer() {
        let solution = day3::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 107069718);
    }
}
