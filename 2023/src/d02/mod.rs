use std::{cmp::max, path::Path};

use crate::util::{read_lines, InputLines};

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn p2(lines: InputLines) {
    let mut sum = 0;

    for line in lines {
        let rline = line.expect("Failed to read line");

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        // Example line:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let words: Vec<&str> = rline.split_whitespace().skip(2).collect();

        for i in (0..words.len()).step_by(2) {
            let n = words[i].parse::<i32>().unwrap();
            let color = words[i + 1].chars().next().unwrap();

            match color {
                'r' => max_r = max(n, max_r),
                'g' => max_g = max(n, max_g),
                'b' => max_b = max(n, max_b),
                _ => panic!("Color not in {{r, g, b}}"),
            }
        }

        sum += max_r * max_g * max_b;
    }

    println!("Part 2: {sum}");
}

fn p1(lines: InputLines) {
    let mut sum = 0;

    for (i, line) in lines.enumerate() {
        let rline = line.expect("Failed to read line");

        // Example line:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let words: Vec<&str> = rline.split_whitespace().skip(2).collect();
        let mut possible = true;

        for j in (0..words.len()).step_by(2) {
            let n = words[j].parse::<i32>().unwrap();
            let color = words[j + 1].chars().next().unwrap();

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
    p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
