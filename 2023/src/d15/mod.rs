use std::path::Path;

use crate::util::get_lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Lens {
    label: String,
    focal_length: i32,
}

#[derive(Debug)]
struct Boxes {
    boxes: Vec<Vec<Lens>>,
}

impl Boxes {
    fn new(len: i32) -> Self {
        let mut boxes: Vec<Vec<Lens>> = vec![];
        for _ in 0..len {
            boxes.push(vec![]);
        }

        Boxes { boxes }
    }

    pub fn remove(&mut self, label: &str, row: usize) {
        let box_row = self.boxes.get_mut(row).unwrap();

        for (i, box_lens) in box_row.iter().enumerate() {
            if box_lens.label == label {
                box_row.remove(i);
                break;
            }
        }
    }

    pub fn add(&mut self, lens: Lens, row: usize) {
        let box_row = self.boxes.get_mut(row).unwrap();

        for (i, box_lens) in box_row.iter().enumerate() {
            if box_lens.label == lens.label {
                box_row[i] = lens;
                return;
            }
        }

        box_row.push(lens);
    }

    pub fn focusing_power(self) -> i32 {
        let mut power: i32 = 0;

        for (i, row) in self.boxes.iter().enumerate() {
            for (j, lens) in row.iter().enumerate() {
                power += (i as i32 + 1) * (j as i32 + 1) * lens.focal_length
            }
        }

        power
    }
}

fn hash(s: &str) -> i32 {
    s.chars().fold(0, |mut acc, c| {
        acc += (c as u8) as i32;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn p2(line: String) {
    let mut boxes = Boxes::new(256);

    for instr in line.split(',') {
        let label: Vec<&str> = instr.split(|c| c == '=' || c == '-').collect();
        let label_hash = hash(label[0]);
        let i = label[0].len();

        match (instr.chars().nth(i).unwrap(), label.get(1)) {
            ('=', Some(v)) => {
                let lens = Lens {
                    label: label[0].to_string(),
                    focal_length: v.parse::<i32>().unwrap(),
                };
                boxes.add(lens, label_hash as usize);
            }
            ('-', _) => boxes.remove(label[0], label_hash as usize),
            _ => panic!(),
        }
    }

    println!("Part 2: {}", boxes.focusing_power());
}

fn p1(line: String) {
    let sum = line.split(',').fold(0, |acc, x| acc + hash(x));
    println!("Part 1: {}", sum);
}

pub fn d15() {
    let input_file = Path::new("src/d15/in.txt");

    p1(get_lines(input_file).first().unwrap().to_string());
    p2(get_lines(input_file).first().unwrap().to_string());
}
