use crate::common::Solution;

#[derive(Debug)]
struct WordSearch {
    characters: Vec<u8>,
    width: usize,
    height: usize,
}

impl WordSearch {
    pub fn from(input: &str) -> Result<Self, String> {
        if input.chars().all(Self::is_valid_char) {
            let characters: Vec<u8> = input
                .bytes()
                .filter(|b| *b != b'\n' && *b != b'\r')
                .collect();
            let height = input.lines().count();
            let width = input.lines().next().ok_or("empty input".to_string())?.len();

            Ok(Self {
                characters,
                width,
                height,
            })
        } else {
            Err("Input contains invalid characters".to_string())
        }
    }

    fn is_valid_char(c: char) -> bool {
        let allowed_chars = ['X', 'M', 'A', 'S', '\r', '\n'];
        allowed_chars.contains(&c)
    }

    fn get_at(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        let index = y * self.height + x;
        Some(self.characters.get(index)?.to_owned() as char)
    }
}

pub fn solve(input: String) -> Result<Solution, String> {
    let word_search = WordSearch::from(&input)?;
    let mut one_star_answer = 0;
    let mut two_star_answer = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for c in word_search.characters.iter() {
        match *c as char {
            'X' => one_star_answer += count_xmas(&word_search, x, y),
            'A' => {
                if is_x_mas(&word_search, x, y) {
                    two_star_answer += 1
                }
            }
            _ => {}
        }

        x += 1;
        if x as usize == word_search.width {
            x = 0;
            y += 1;
        }
    }

    Ok(Solution {
        one_star_answer,
        two_star_answer,
    })
}

fn count_xmas(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> i32 {
    if word_search.get_at(initial_x, initial_y) != Some('X') {
        return 0;
    }

    [
        search_xmas_north,
        search_xmas_north_east,
        search_xmas_east,
        search_xmas_south_east,
        search_xmas_south,
        search_xmas_south_west,
        search_xmas_west,
        search_xmas_north_west,
    ]
    .iter()
    .filter(|f| f(word_search, initial_x, initial_y))
    .count() as i32
}

fn is_x_mas(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    if word_search.get_at(initial_x, initial_y) != Some('A') {
        return false;
    }

    let top_left = word_search.get_at(initial_x - 1, initial_y - 1);
    let top_right = word_search.get_at(initial_x + 1, initial_y - 1);
    let bot_left = word_search.get_at(initial_x - 1, initial_y + 1);
    let bot_right = word_search.get_at(initial_x + 1, initial_y + 1);

    if let Some(top_left) = top_left {
        if let Some(top_right) = top_right {
            if let Some(bot_left) = bot_left {
                if let Some(bot_right) = bot_right {
                    let arm1 = format!("{top_left}{bot_right}");
                    let arm2 = format!("{top_right}{bot_left}");
                    return arm1.contains('M')
                        && arm1.contains('S')
                        && arm2.contains('M')
                        && arm2.contains('S');
                }
            }
        }
    }
    false
}

fn search_xmas_north(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x, initial_y - 1) == Some('M')
        && word_search.get_at(initial_x, initial_y - 2) == Some('A')
        && word_search.get_at(initial_x, initial_y - 3) == Some('S')
}

fn search_xmas_north_east(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x + 1, initial_y - 1) == Some('M')
        && word_search.get_at(initial_x + 2, initial_y - 2) == Some('A')
        && word_search.get_at(initial_x + 3, initial_y - 3) == Some('S')
}

fn search_xmas_east(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x + 1, initial_y) == Some('M')
        && word_search.get_at(initial_x + 2, initial_y) == Some('A')
        && word_search.get_at(initial_x + 3, initial_y) == Some('S')
}

fn search_xmas_south_east(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x + 1, initial_y + 1) == Some('M')
        && word_search.get_at(initial_x + 2, initial_y + 2) == Some('A')
        && word_search.get_at(initial_x + 3, initial_y + 3) == Some('S')
}

fn search_xmas_south(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x, initial_y + 1) == Some('M')
        && word_search.get_at(initial_x, initial_y + 2) == Some('A')
        && word_search.get_at(initial_x, initial_y + 3) == Some('S')
}

fn search_xmas_south_west(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x - 1, initial_y + 1) == Some('M')
        && word_search.get_at(initial_x - 2, initial_y + 2) == Some('A')
        && word_search.get_at(initial_x - 3, initial_y + 3) == Some('S')
}

fn search_xmas_west(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x - 1, initial_y) == Some('M')
        && word_search.get_at(initial_x - 2, initial_y) == Some('A')
        && word_search.get_at(initial_x - 3, initial_y) == Some('S')
}

fn search_xmas_north_west(word_search: &WordSearch, initial_x: i32, initial_y: i32) -> bool {
    word_search.get_at(initial_x - 1, initial_y - 1) == Some('M')
        && word_search.get_at(initial_x - 2, initial_y - 2) == Some('A')
        && word_search.get_at(initial_x - 3, initial_y - 3) == Some('S')
}

#[cfg(test)]
mod tests {
    use crate::day4::WordSearch;
    use crate::{common, day4};

    fn real_input() -> String {
        common::get_input(4, "resources/inputs").expect("Cannot get input")
    }

    fn sample_input() -> String {
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
            .to_string()
    }

    #[test]
    pub fn parse_input() {
        let invalid_input = "XMASQ".to_string();
        WordSearch::from(&invalid_input).expect_err("expected Err(), got Ok(WordSearch)");

        let word_search =
            WordSearch::from(&sample_input()).expect("expected Ok(WordSearch), got Err()");

        assert_eq!(word_search.height, 10);
        assert_eq!(word_search.width, 10);
    }

    #[test]
    pub fn get_at() {
        let word_search =
            WordSearch::from(&sample_input()).expect("expected Ok(WordSearch), got Err()");

        assert_eq!(word_search.get_at(0, 1), Some('M'));
        assert_eq!(word_search.get_at(1, 1), Some('S'));
        assert_eq!(word_search.get_at(0, 2), Some('A'));
        assert_eq!(word_search.get_at(9, 1), Some('A'));
        assert_eq!(word_search.get_at(0, 9), Some('M'));
        assert_eq!(word_search.get_at(4, 9), Some('A'));
    }

    #[test]
    pub fn sample_input_one_star_answer() {
        let solution = day4::solve(sample_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 18);
    }

    #[test]
    pub fn one_star_answer() {
        let solution = day4::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.one_star_answer, 2458);
    }

    #[test]
    pub fn sample_input_two_star_answer() {
        let solution = day4::solve(sample_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 9);
    }

    #[test]
    pub fn two_star_answer() {
        let solution = day4::solve(real_input()).expect("expected Ok(Solution), got Err()");
        assert_eq!(solution.two_star_answer, 1945);
    }
}
