use crate::util::get_lines;

type Filesys = Vec<Option<i64>>;

fn make_filesys(s: &str) -> Filesys {
    let mut filesys: Filesys = Vec::with_capacity(s.len());

    let mut id = 0;
    for (i, ch) in s.char_indices() {
        let digit = ch.to_digit(10).unwrap();

        if i % 2 == 0 {
            for _ in 0..digit {
                filesys.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..digit {
                filesys.push(None);
            }
        }
    }

    filesys
}

fn next_gap(fs: &Filesys) -> Option<usize> {
    let mut gap_idx = None;

    for i in 0..fs.len() {
        if fs[i].is_none() && gap_idx.is_none() {
            gap_idx = Some(i);
        } else if fs[i].is_some() && gap_idx.is_some() {
            return gap_idx;
        }
    }

    None
}

fn next_gap_of_size() -> Option<usize> {
    todo!()
}

fn checksum(fs: &Filesys) -> i64 {
    let mut sum = 0;
    for i in 0..fs.len() {
        if let Some(v) = fs[i] {
            sum += i as i64 * v;
        }
    }
    sum
}

fn p2(lines: Vec<String>) {
    let mut fs = make_filesys(&lines[0]);
    let mut empty_idx = next_gap(&fs);

    let mut i = 0;
    while i > 0 {
        if fs[i].is_none() {
            i -= 1;
            continue;
        }

        let block_start = i;
        let mut block_size = 1;
        while fs[i].is_some_and(|x| x == fs[block_start].unwrap()) {
            block_size += 1;
            i -= 1;
        }

        i -= 1;
    }

    println!("{}", checksum(&fs));
}

fn p1(lines: Vec<String>) {
    let mut fs = make_filesys(&lines[0]);
    let mut empty_idx = next_gap(&fs);

    for i in (0..fs.len()).rev() {
        if fs[i].is_some() {
            if let Some(idx) = empty_idx {
                fs[idx] = fs[i];
                fs[i] = None;
                empty_idx = next_gap(&fs);
            } else {
                break;
            }
        }
    }

    println!("{}", checksum(&fs));
}

pub fn d09() {
    p1(get_lines("src/d09/dbg.txt"));
    p2(get_lines("src/d09/dbg.txt"));
}
