use crate::util::get_lines;

fn look_ahead(s: &str, ch: char, max: usize) -> Option<usize> {
    let ub = max.min(s.len() - 1);
    let slice = &s[..=ub];
    slice.find(ch)
}

fn mul_nums(line: &str, i: usize) -> Option<(i32, i32)> {
    let comma = look_ahead(&line[i..], ',', 4);
    let closing = look_ahead(&line[i..], ')', 7);

    match (comma, closing) {
        (None, None) | (_, None) | (None, _) => None,
        (Some(comma_i), Some(closing_i)) => {
            let num1 = line[i..i + comma_i].parse::<i32>().unwrap();
            let num2 = line[i + comma_i + 1..i + closing_i].parse::<i32>().unwrap();
            Some((num1, num2))
        }
    }
}

pub fn p02(lines: Vec<String>) {
    let mut sum = 0;
    let mul = "mul(";
    let doo = "do()";
    let dont = "don't()";

    let mut is_mul = true;
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();

        let mut i = 0;
        while i < chars.len() {
            match (chars[i], is_mul) {
                ('m', true) => {
                    if line[i..i + mul.len()].contains(mul) {
                        let nums = mul_nums(&line, i + mul.len());
                        if let Some(n) = nums {
                            sum += n.0 * n.1;
                        }
                        i += mul.len() - 1;
                    }
                }
                ('d', _) => {
                    if line[i..i + doo.len()].contains(doo) {
                        is_mul = true;
                        i += doo.len() - 1;
                    } else if line[i..i + dont.len()].contains(dont) {
                        is_mul = false;
                        i += dont.len() - 1;
                    }
                }
                _ => (),
            }

            i += 1;
        }
    }

    println!("{}", sum);
}

pub fn p01(lines: Vec<String>) {
    let mut matched = 0;
    let mut sum = 0;

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            match (ch, matched) {
                ('m', 0) => matched = 1,
                ('u', 1) => matched = 2,
                ('l', 2) => matched = 3,
                ('(', 3) => matched = 4,
                (_, 4) => {
                    let nums = mul_nums(&line, i);
                    if let Some(n) = nums {
                        sum += n.0 * n.1;
                    }
                    matched = 0;
                }
                _ => matched = 0,
            }
        }
    }

    println!("{}", sum);
}

pub fn d03() {
    p01(get_lines("src/d03/in.txt"));
    p02(get_lines("src/d03/in.txt"));
}
