use std::{
    fs::File,
    io::{BufReader, Lines},
    path::Path,
};

use crate::util::get_lines;

fn get_races(lines: &Vec<String>) -> Vec<(i32, i32)> {
    let mut races: Vec<(i32, i32)> = vec![];

    for t in lines[0].split_whitespace().skip(1) {
        races.push((t.parse::<i32>().unwrap(), -1));
    }

    for (i, t) in lines[1].split_whitespace().skip(1).enumerate() {
        let race = races.get_mut(i).unwrap();
        race.1 = t.parse::<i32>().unwrap();
    }

    races
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let races = get_races(&lines);
    let mut sum = -1;

    for race in races {
        let mut ways_to_beat = 0;

        for i in 0..=(race.0 / 2) {
            let dist = (race.0 - i) * i;

            if dist > race.1 {
                ways_to_beat += 1;
            }
        }

        ways_to_beat *= 2;
        if race.0 % 2 == 0 {
            ways_to_beat -= 1;
        }

        match sum {
            -1 => sum = ways_to_beat,
            _ => sum *= ways_to_beat,
        }
    }

    println!("Part 1: {}", sum);
}

pub fn d06() {
    let input_file = Path::new("src/d06/in.txt");

    p1(get_lines(input_file));
    // p2(read_lines(input_file).expect("Couldnt read lines of file"));
}
