use std::{cmp::Ordering, collections::HashMap};

use crate::util::get_lines;

fn make_order_map(lines: &[String]) -> HashMap<i32, Vec<i32>> {
    let mut map = HashMap::new();

    for line in lines {
        let (n1, n2) = line.split_once("|").unwrap();
        let earlier = n1.parse::<i32>().unwrap();
        let later = n2.parse::<i32>().unwrap();

        map.entry(earlier).or_insert(vec![]).push(later);
    }

    map
}

fn is_correct_order(nums: &[i32], map: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if !is_earlier(nums[i], nums[j], map) {
                return false;
            }
        }
    }

    true
}

fn is_earlier(a: i32, b: i32, map: &HashMap<i32, Vec<i32>>) -> bool {
    map.get(&a).map_or(false, |vc| vc.contains(&b))
}

fn p01(lines: Vec<String>) {
    let mut sum = 0;
    let mut i = lines.iter().position(|e| e.is_empty()).unwrap();
    let map = make_order_map(&lines[..i]);

    i += 1;

    while i < lines.len() {
        let nums = lines[i]
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_correct_order(&nums, &map) {
            sum += nums[nums.len() / 2];
        }

        i += 1;
    }

    println!("{}", sum);
}

fn p02(lines: Vec<String>) {
    let mut sum = 0;
    let mut i = lines.iter().position(|e| e.is_empty()).unwrap();
    let map = make_order_map(&lines[..i]);

    i += 1;

    while i < lines.len() {
        let mut nums = lines[i]
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if !is_correct_order(&nums, &map) {
            nums.sort_by(|a, b| match is_earlier(*a, *b, &map) {
                true => Ordering::Greater,
                false => Ordering::Less,
            });

            sum += nums[nums.len() / 2];
        }

        i += 1;
    }

    println!("{}", sum);
}

pub fn d05() {
    p01(get_lines("src/d05/in.txt"));
    p02(get_lines("src/d05/in.txt"));
}
