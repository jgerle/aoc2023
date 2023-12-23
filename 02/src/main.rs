use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // open file and read line by line - file must exist in the current path
    if let Ok(lines) = read_lines("./sample.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(game_raw) = line {
                let game = process_line_with_game(&game_raw);
                println!("{:?}", game);
            }
        }
    }
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

#[allow(dead_code)]
struct Games {
    list: Vec<Game>,
}
#[derive(Debug)]
#[allow(dead_code)]
struct Game {
    id: u32,
    cubesets: Option<Vec<Cubeset>>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Cubeset {
    red: u32,
    green: u32,
    blue: u32,
}
impl Default for Cubeset {
    fn default() -> Cubeset {
        Cubeset {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

/* Split the line in a game part and rounds */
fn process_line_with_game(line: &String) -> Option<Game> {
    // split line by colon to have game meta info and cube sets
    let parts = line.split(": ").collect::<Vec<&str>>();
    let mut parts_iter = parts.iter();
    let id_part = parts_iter.next();
    let cubesets_part = parts_iter.next();

    let game = Game {
        id: get_id(id_part),
        cubesets: get_cubesets(cubesets_part),
    };

    // TODO: process game cube sets
    return Some(game);
}

/* Get the id of a game from a line */
fn get_id(id_part: Option<&&str>) -> u32 {
    match id_part {
        Some(id_part) => {
            // split by whitespace, get the second part
            let id_part_string = id_part.to_string();
            let id_part_parts = id_part_string.split(" ").collect::<Vec<&str>>();

            let id_part_parts_iter = id_part_parts.iter();
            let mut id_skip_iter = id_part_parts_iter.skip(1);
            let id = id_skip_iter.next();

            match id {
                Some(id) => id.parse::<u32>().expect("Part should contain id"),
                None => panic!("No id found in id_part"),
            }
        }
        None => panic!("No id part found in input"),
    }
    //;
}

/// Extract cube sets from String and have them parsed into array of structs
/* Input data: " 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" */
fn get_cubesets(cubesets_part: Option<&&str>) -> Option<Vec<Cubeset>> {
    match cubesets_part {
        Some(cubeset_line) => {
            let mut cubesets: Vec<Cubeset> = Vec::new();
            let input_parts = cubeset_line.split("; ").collect::<Vec<&str>>();

            if input_parts.len() > 0 {
                for cubeset_raw in input_parts.iter() {
                    match get_cubeset(cubeset_raw) {
                        Some(cubeset) => {
                            cubesets.push(cubeset);
                        }
                        None => {
                            panic!("No cubeset found! Malformed input?")
                        }
                    }
                }
                return Some(cubesets);
            } else {
                return None;
            }
        }
        None => return None,
    }
}

/* Input data example: "6 red, 1 blue, 3 green" */
fn get_cubeset(cubeset_raw: &str) -> Option<Cubeset> {
    let cubeset_parts = cubeset_raw.split(", ").collect::<Vec<&str>>(); // elements be like "6 red", "1 blue" and "3 green"
    let cubeset_parts_iter = cubeset_parts.iter();
    let mut cubeset = Cubeset::default();
    for cubeset_raw in cubeset_parts_iter {
        let cubeset_raw_parts = cubeset_raw.split(" ").collect::<Vec<&str>>(); // now we have e.g. "6" and "red"
        let mut cubeset_raw_parts_iter = cubeset_raw_parts.iter();
        let cubeset_raw_part_count = cubeset_raw_parts_iter.next()?.parse::<u32>().unwrap();
        let cubeset_raw_part_color = cubeset_raw_parts_iter.next().unwrap().to_string();
        match cubeset_raw_part_color.as_str() {
            "red" => cubeset.red = cubeset_raw_part_count,
            "blue" => cubeset.blue = cubeset_raw_part_count,
            "green" => cubeset.green = cubeset_raw_part_count,
            _ => panic!("Unknown color found!"),
        }
    }
    //let trimmed_input = cubeset_parts.trim();
    return Some(cubeset);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_id_five() {
        let value = "Game 5";
        let input: Option<&&str> = Some(&value);
        let five: u32 = get_id(input);

        assert_eq!(5, five);
    }

    #[test]
    fn get_6red_from_cubeset() {
        let sampledata = "6 red, 1 blue, 3 green";
        let cubeset = get_cubeset(sampledata);
        assert_eq!(6, cubeset.unwrap().red);
    }
    #[test]
    fn get_1blue_from_cubeset() {
        let sampledata = "6 red, 1 blue, 3 green";
        let cubeset = get_cubeset(sampledata);
        assert_eq!(1, cubeset.unwrap().blue);
    }
    #[test]
    fn get_3green_from_cubeset() {
        let sampledata = "6 red, 1 blue, 3 green";
        let cubeset = get_cubeset(sampledata);
        assert_eq!(3, cubeset.unwrap().green);
    }

    #[test]
    fn process_line() {
        let line = String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let game = process_line_with_game(&line);
        assert_eq!(5, game.unwrap().id);
    }
}