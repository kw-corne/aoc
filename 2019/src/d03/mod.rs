use core::panic;
use std::{collections::HashMap, collections::HashSet, path::Path};

use crate::util::get_lines;

// TODO: Only use steps_to_point
struct Wire {
    points: HashSet<[i32; 2]>,
    steps_to_point: HashMap<[i32; 2], i32>,
}

impl Wire {
    fn new(input_line: &str) -> Self {
        let mut points: HashSet<[i32; 2]> = HashSet::new();
        let mut steps_to_point: HashMap<[i32; 2], i32> = HashMap::new();
        let mut curr_point = [0, 0];
        points.insert(curr_point);

        let mut steps_taken = 0;

        for instr in input_line.split(',') {
            let dir = instr.chars().nth(0).unwrap();
            let n = &instr[1..].parse::<i32>().unwrap();

            let mut dx = 0;
            let mut dy = 0;

            match dir {
                'R' => dx = 1,
                'L' => dx = -1,
                'U' => dy = 1,
                'D' => dy = -1,
                _ => panic!(),
            }

            for _ in 0..*n {
                steps_to_point.insert(curr_point, steps_taken);
                steps_taken += 1;
                points.insert(curr_point.clone());

                curr_point[0] += dx;
                curr_point[1] += dy;
            }
        }

        Self {
            points,
            steps_to_point,
        }
    }

    fn intersection(&self, other: &Wire) -> HashSet<[i32; 2]> {
        self.points
            .intersection(&other.points)
            .cloned()
            .filter(|&p| p != [0, 0])
            .collect()
    }
}

fn dist_to_origin(x: &[i32; 2]) -> i32 {
    (x[0] - 0).abs() + (x[1] - 0).abs()
}

fn p2(lines: Vec<String>) {
    let wire1 = Wire::new(&lines[0]);
    let wire2 = Wire::new(&lines[1]);

    let intersection = wire1.intersection(&wire2);

    let mut min_point = i32::MAX;
    for point in intersection {
        let steps1 = wire1.steps_to_point.get(&point).unwrap();
        let steps2 = wire2.steps_to_point.get(&point).unwrap();

        min_point = min_point.min(steps1 + steps2);
    }

    println!("{:?}", min_point);
}

fn p1(lines: Vec<String>) {
    let wire1 = Wire::new(&lines[0]);
    let wire2 = Wire::new(&lines[1]);

    let intersection = wire1.intersection(&wire2);
    let closest = intersection
        .iter()
        .min_by(|x, y| dist_to_origin(&x).cmp(&dist_to_origin(&y)))
        .map(|&p| dist_to_origin(&p))
        .unwrap();

    println!("{:?}", closest);
}

pub fn d03() {
    let input_file = Path::new("src/d03/in.txt");

    // p1(get_lines(input_file));
    p2(get_lines(input_file));
}
