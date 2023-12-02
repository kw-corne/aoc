use std::path::Path;

use crate::util::{read_lines, InputLines};

fn p2(lines: InputLines) {
    todo!();
}

fn p1(lines: InputLines) {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut sum = 0;

    for (i, line) in lines.enumerate() {
        let rline = line.expect("Failed to read line");

        // Example line:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let words: Vec<&str> = rline.split_whitespace().skip(2).collect();
        let mut possible = true;

        for i in (0..words.len()).step_by(2) {
            let n = words[i].parse::<i32>().unwrap();
            let color = words[i + 1].chars().next().unwrap();

            match color {
                'r' => {
                    if n > MAX_RED {
                        possible = false;
                    }
                }
                'g' => {
                    if n > MAX_GREEN {
                        possible = false;
                    }
                }
                'b' => {
                    if n > MAX_BLUE {
                        possible = false;
                    }
                }
                _ => panic!("Color not in {{r, g, b}}"),
            }

            if !possible {
                break;
            }
        }

        if possible {
            sum += i + 1; // game starts at 1, i at 0
        }
    }

    println!("Part 1: {sum}");
}

pub fn d02() {
    let input_file = Path::new("src/d02/in.txt");

    p1(read_lines(input_file).expect("Couldnt read lines of file"));
    // p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
