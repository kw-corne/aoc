use std::cmp::{max, min};
use std::{collections::HashMap, path::Path};

use crate::util::get_lines;

#[derive(Debug)]
struct Rule {
    conds: Vec<(char, char, i32, String)>,
    last: String,
}

impl Rule {
    fn new(line: &str) -> Self {
        let left_bracket = line.find('{').unwrap();
        let right_bracket = line.find('}').unwrap();

        let mut last = "".to_string();
        let mut conds: Vec<(char, char, i32, String)> = vec![];
        let in_bracket = &line[left_bracket + 1..right_bracket];

        for e in in_bracket.split(',') {
            let s: Vec<&str> = e.split(':').collect();

            match s.len() {
                1 => last = e.to_string(),
                _ => {
                    let cat = s[0].chars().nth(0).unwrap();
                    let sign = s[0].chars().nth(1).unwrap();
                    let num = s[0][2..].parse::<i32>().unwrap();
                    let dest = s[1].to_string();

                    conds.push((cat, sign, num, dest));
                }
            }
        }

        assert_ne!(last, "");
        Self { conds, last }
    }
}

fn permutations(item: &[(i32, i32); 4]) -> i64 {
    let mut sum: i64 = 1;
    for v in item {
        sum *= (v.1 - v.0 + 1) as i64;
    }
    sum
}

fn valid_range(range: &(i32, i32)) -> bool {
    range.0 < range.1
}

const MAX_VAL: i32 = 4000;

fn h(s: &str, vals: &[(i32, i32); 4], rules: &HashMap<String, Rule>) -> i64 {
    if s == "A" {
        return permutations(vals);
    } else if s == "R" {
        return 0;
    }

    let rule = rules.get(s).unwrap();
    let mut leftover = vals.clone();
    let mut curr_range = vals.clone();
    let mut sum: i64 = 0;

    for cond in &rule.conds {
        let i = {
            match cond.0 {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!(),
            }
        };

        let new_range;
        match cond.1 {
            '<' => {
                new_range = (curr_range[i].0, min(cond.2 - 1, curr_range[i].1));
                leftover[i].0 = max(leftover[i].0, cond.2);
            }
            '>' => {
                new_range = (max(cond.2 + 1, curr_range[i].0), curr_range[i].1);
                leftover[i].1 = min(leftover[i].1, cond.2);
            }
            _ => panic!(),
        }

        if valid_range(&new_range) {
            let mut cr = curr_range.clone();
            cr[i] = new_range;
            sum += h(&cond.3, &cr, rules);
        }

        curr_range[i] = leftover[i];
    }

    if leftover.iter().all(|r| valid_range(r)) {
        sum += h(&rule.last, &leftover, rules);
    }

    sum
}

fn p2(lines: Vec<String>) {
    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut i = 0;

    while lines[i] != "" {
        let name = &lines[i][..lines[i].find('{').unwrap()];
        rules.insert(name.to_string(), Rule::new(&lines[i]));
        i += 1;
    }

    println!(
        "Part 2: {}",
        h(
            "in",
            &[(1, MAX_VAL), (1, MAX_VAL), (1, MAX_VAL), (1, MAX_VAL)],
            &rules
        )
    );
}

fn p1(lines: Vec<String>) {
    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut items: Vec<[i32; 4]> = vec![];
    let mut i = 0;

    while lines[i] != "" {
        let name = &lines[i][..lines[i].find('{').unwrap()];
        rules.insert(name.to_string(), Rule::new(&lines[i]));
        i += 1;
    }
    i += 1;

    while i < lines.len() {
        let mut nums = [0; 4];
        for (i, e) in lines[i][1..lines[i].len() - 1].split(',').enumerate() {
            nums[i] = e[2..].parse::<i32>().unwrap();
        }
        items.push(nums);
        i += 1;
    }

    let mut sum = 0;
    for item in items {
        let mut curr_rule = "in".to_string();
        while curr_rule != "A" && curr_rule != "R" {
            let rule = rules.get(&curr_rule).unwrap();
            let mut p = false;
            for cond in &rule.conds {
                let val = {
                    match cond.0 {
                        'x' => item[0],
                        'm' => item[1],
                        'a' => item[2],
                        's' => item[3],
                        _ => panic!(),
                    }
                };

                let passes = {
                    match cond.1 {
                        '<' => val < cond.2,
                        '>' => val > cond.2,
                        _ => panic!(),
                    }
                };

                if passes {
                    curr_rule = cond.3.clone();
                    p = true;
                    break;
                }
            }
            if !p {
                curr_rule = rule.last.clone();
            }
        }
        if curr_rule == "A" {
            sum += item.iter().sum::<i32>();
        }
    }

    println!("Part 1: {}", sum);
}

pub fn d19() {
    let input_file = Path::new("src/d19/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
