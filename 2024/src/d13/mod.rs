use std::collections::HashMap;

use crate::util::get_lines;

#[derive(Debug)]
struct Machine {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    px: f64,
    py: f64,
}

impl Machine {
    fn new(lines: &[String]) -> Self {
        let split_a = lines[0].split_whitespace().collect::<Vec<_>>();
        let split_b = lines[1].split_whitespace().collect::<Vec<_>>();
        let split_p = lines[2].split_whitespace().collect::<Vec<_>>();

        Machine {
            ax: Self::to_num(split_a[2]),
            ay: Self::to_num(split_a[3]),
            bx: Self::to_num(split_b[2]),
            by: Self::to_num(split_b[3]),
            px: Self::to_num(split_p[1]),
            py: Self::to_num(split_p[2]),
        }
    }

    fn to_num(s: &str) -> f64 {
        s.chars()
            .skip(2)
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<f64>()
            .unwrap()
    }

    fn cheapest(&self, offset: i64) -> Option<i64> {
        let px = self.px + offset as f64;
        let py = self.py + offset as f64;

        let det = (self.ax * self.by) - (self.ay * self.bx);
        let det_b = (px * self.by) - (py * self.bx);
        let det_a = (px * self.ay) - (py * self.ax);

        let a = det_b / det;
        let b = det_a / det;

        if a.fract() == 0.0 && b.fract() == 0.0 {
            return Some(a.abs() as i64 * 3 + b.abs() as i64);
        }

        None
    }
}

fn p02(lines: Vec<String>) {
    let mut i = 0;
    let mut sum = 0;

    while i < lines.len() {
        let machine = Machine::new(&lines[i..i + 3]);
        sum += machine.cheapest(10000000000000).unwrap_or(0);
        i += 4;
    }

    println!("{}", sum);
}

fn p01(lines: Vec<String>) {
    let mut i = 0;
    let mut sum = 0;

    while i < lines.len() {
        let machine = Machine::new(&lines[i..i + 3]);
        sum += machine.cheapest(0).unwrap_or(0);
        i += 4;
    }

    println!("{}", sum);
}

pub fn d13() {
    p01(get_lines("src/d13/in.txt"));
    p02(get_lines("src/d13/in.txt"));
}
