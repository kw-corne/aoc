use itertools::{iproduct, Itertools};

use crate::util::get_lines;

fn p02(lines: Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let split = line.split_once(':').unwrap();

        let target = split.0.parse::<i64>().unwrap();
        let nums = &split.1[1..]
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let combis: Vec<_> = (0..nums.len() - 1)
            .map(|_| 0..3)
            .multi_cartesian_product()
            .collect();

        for comb in combis {
            let mut row_sum = nums[0];

            for j in 1..nums.len() {
                match comb[j - 1] {
                    0 => row_sum += nums[j],
                    1 => row_sum *= nums[j],
                    2 => row_sum = format!("{}{}", row_sum, nums[j]).parse::<i64>().unwrap(),
                    _ => panic!(),
                }
            }

            if row_sum == target {
                sum += target;
                break;
            }
        }
    }

    println!("{}", sum);
}

fn p01(lines: Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let split = line.split_once(':').unwrap();

        let target = split.0.parse::<i64>().unwrap();
        let nums = &split.1[1..]
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let multi_idxs: Vec<usize> = (0..=(nums.len() - 2)).collect();

        'out: for i in 0..=multi_idxs.len() {
            for multi_is in multi_idxs.iter().combinations(i) {
                let mut row_sum = nums[0];

                for j in 1..nums.len() {
                    if multi_is.contains(&&(j - 1)) {
                        row_sum *= nums[j];
                    } else {
                        row_sum += nums[j];
                    }
                }

                if row_sum == target {
                    sum += target;
                    break 'out;
                }
            }
        }
    }

    println!("{}", sum);
}

pub fn d07() {
    p01(get_lines("src/d07/in.txt"));
    p02(get_lines("src/d07/in.txt"));
}
