use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod config;

/* Examples:
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
Adding these together produces 281.
 */
fn main() {
    // get config (list of digits as strings)
    let needles = config::init();
    let mut final_result: u32 = 0;

    // open file and read line by line - file must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(haystack) = line {
                // find digit-strings in each line
                let mut min_position: Option<u32> = None; // track lowest position found
                let mut min_digit: Option<u32> = None; // track digit found on lowest position here; remains None if no digit found
                let mut max_position: Option<u32> = None; // track highest position found
                let mut max_digit: Option<u32> = None; // track digit found on highest position here; remains None if no digit found

                for (digit, value) in &needles {
                    let occurences: Vec<_> = haystack.match_indices(digit).collect();

                    for position in occurences.iter() {
                        // check and set low
                        match min_position {
                            Some(curr_min_val) => {
                                if (position.0 as u32) < curr_min_val {
                                    min_position = Some(position.0 as u32);
                                    min_digit = Some(*value);
                                }
                            }
                            None => {
                                min_position = Some(position.0 as u32);
                                min_digit = Some(*value);
                            }
                        }
                        // check and set high
                        match max_position {
                            Some(curr_max_val) => {
                                if (position.0 as u32) > curr_max_val {
                                    max_position = Some(position.0 as u32);
                                    max_digit = Some(*value);
                                }
                            }
                            None => {
                                max_position = Some(position.0 as u32);
                                max_digit = Some(*value);
                            }
                        }
                    }
                }
                let line_result = get_line_result(&min_digit, &max_digit);

                final_result += line_result;
            }
        }
    }
    println!("Final result: {final_result}")
}

fn get_line_result(min_digit: &Option<u32>, max_digit: &Option<u32>) -> u32 {
    let s1 = min_digit.unwrap().to_string();
    let s2 = max_digit.unwrap().to_string();

    let combo = s1.to_string() + &s2;
    return combo.to_string().parse::<u32>().unwrap();
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_three_as_first() {
        let config: HashMap<&str, i32> = config::init();
        let three: u32 = find_first_digit(&config, String::from("threepqr4stu8vwx"));

        assert_eq!(3, three);
    }
    #[test]
    fn get_eight_as_last() {
        let eight = find_last_digit(String::from("pqr3stu8vwx"));
        if let Some(val) = eight {
            assert_eq!(8, val);
        } else {
            panic!();
        }
    }
}
