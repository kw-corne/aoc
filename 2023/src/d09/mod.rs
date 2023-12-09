use std::path::Path;

use crate::util::get_lines;

fn p2(lines: Vec<String>) {
    let mut sum = 0;

    for line in &lines {
        let mut seqs: Vec<Vec<i32>> = vec![];
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        seqs.push(nums);

        let mut cv = 0;
        loop {
            let mut diff_vec: Vec<i32> = vec![];

            for j in 0..seqs[cv].len() - 1 {
                diff_vec.push(seqs[cv][j + 1] - seqs[cv][j]);
            }

            seqs.push(diff_vec);
            cv += 1;

            if seqs[cv].iter().all(|&x| x == 0) {
                break;
            }
        }

        let mut last_val = 0;

        for i in (0..seqs.len() - 1).rev() {
            let n = seqs[i].first().unwrap() - last_val;

            if i > 0 {
                last_val = n;
            } else {
                sum += n;
            }
        }
    }

    println!("Part 2: {}", sum);
}

fn p1(lines: Vec<String>) {
    let mut sum = 0;

    for line in &lines {
        let mut seqs: Vec<Vec<i32>> = vec![];
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        seqs.push(nums);

        let mut cv = 0;
        loop {
            let mut diff_vec: Vec<i32> = vec![];

            for j in 0..seqs[cv].len() - 1 {
                diff_vec.push(seqs[cv][j + 1] - seqs[cv][j]);
            }

            seqs.push(diff_vec);
            cv += 1;

            if seqs[cv].iter().all(|&x| x == 0) {
                break;
            }
        }

        seqs.last_mut().unwrap().push(0);

        for i in (1..seqs.len() - 1).rev() {
            let n = seqs[i - 1].last().unwrap() + seqs[i].last().unwrap();
            seqs[i - 1].push(n);
        }

        sum += seqs[0].last().unwrap();
    }

    println!("Part 1: {}", sum);
}

pub fn d09() {
    let input_file = Path::new("src/d09/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
