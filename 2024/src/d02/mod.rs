use crate::util::get_lines;

fn is_safe(nums: &[i32], increasing: bool) -> bool {
    let range = if increasing { 1..=3 } else { -3..=-1 };
    let mut ok = true;

    for i in 0..nums.len() - 1 {
        let mut diff = nums[i] - nums[i + 1];
        diff *= -1;

        if !range.contains(&diff) {
            ok = false;
            break;
        }
    }

    ok
}

fn p02(lines: Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let nums = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for increasing in [true, false] {
            if is_safe(&nums, increasing) {
                sum += 1;
                break;
            } else {
                for i in 0..nums.len() {
                    let without_idx = nums
                        .iter()
                        .cloned()
                        .enumerate()
                        .filter_map(|(idx, v)| if idx == i { None } else { Some(v) })
                        .collect::<Vec<i32>>();

                    if is_safe(&without_idx, increasing) {
                        sum += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", sum)
}

fn p01(lines: Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let nums = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let increasing = nums[0] < nums[1];

        if is_safe(&nums, increasing) {
            sum += 1;
        }
    }

    println!("{}", sum)
}

pub fn d02() {
    p01(get_lines("src/d02/in.txt"));
    p02(get_lines("src/d02/in.txt"));
}
