use std::{collections::HashSet, path::Path};

use crate::util::{get_lines, lines_to_2d_chars};

#[derive(Debug)]
struct Beam {
    pos: (i32, i32),
    dir: (i32, i32),
    prev_dir: Option<(i32, i32)>,
}

impl Beam {
    fn new(pos: (i32, i32), dir: (i32, i32)) -> Self {
        Beam {
            pos,
            dir,
            prev_dir: None,
        }
    }
}

fn in_grid(p: (i32, i32), i: i32, j: i32) -> bool {
    (p.0 >= 0 && p.0 < i) && (p.1 >= 0 && p.1 < j)
}

fn mirr_tup_r(tup: &mut (i32, i32)) {
    let t0 = tup.0;
    let t1 = tup.1;
    tup.0 = t1 * -1;
    tup.1 = t0 * -1;
}

fn mirr_tup_l(tup: &mut (i32, i32)) {
    let t0 = tup.0;
    let t1 = tup.1;
    tup.0 = t1;
    tup.1 = t0;
}

fn walk_beam(
    beams: &mut Vec<Beam>,
    i: i32,
    grid: &Vec<Vec<char>>,
    cache: &mut HashSet<((i32, i32), (i32, i32))>,
) -> Vec<(i32, i32)> {
    let beam = beams.get_mut(i as usize).unwrap();
    let mut new_beams: Vec<Beam> = vec![];
    let mut visited: Vec<(i32, i32)> = vec![];

    while in_grid(beam.pos, grid.len() as i32, grid[0].len() as i32) {
        visited.push(beam.pos.clone());
        if cache.contains(&(beam.pos, beam.dir)) {
            break;
        }
        cache.insert((beam.pos, beam.dir));

        let gc = grid[beam.pos.0 as usize][beam.pos.1 as usize];
        match (gc, beam.dir) {
            ('.', _) | ('|', (1, 0)) | ('|', (-1, 0)) | ('-', (0, 1)) | ('-', (0, -1)) => (),

            ('|', _) => {
                new_beams.push(Beam::new(beam.pos.clone(), (-1, 0)));
                new_beams.push(Beam::new(beam.pos.clone(), (1, 0)));
                break;
            }
            ('-', _) => {
                new_beams.push(Beam::new(beam.pos.clone(), (0, -1)));
                new_beams.push(Beam::new(beam.pos.clone(), (0, 1)));
                break;
            }

            ('/', _) => mirr_tup_r(&mut beam.dir),
            ('\\', _) => mirr_tup_l(&mut beam.dir),

            (c, d) => panic!("Unhandled case: {} {:?}", c, d),
        }
        beam.pos.0 += beam.dir.0 as i32;
        beam.pos.1 += beam.dir.1 as i32;
    }

    beams.remove(0);
    beams.append(&mut new_beams);

    visited
}

fn p2(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);

    let left_col: Vec<Beam> = (0..grid.len())
        .map(|i| Beam::new((i as i32, 0), (0, 1)))
        .collect();

    let right_col: Vec<Beam> = (0..grid.len())
        .map(|i| Beam::new((i as i32, grid[0].len() as i32 - 1), (0, -1)))
        .collect();

    let top_row: Vec<Beam> = (0..grid[0].len())
        .map(|i| Beam::new((0, i as i32), (1, 0)))
        .collect();

    let bot_row: Vec<Beam> = (0..grid[0].len())
        .map(|i| Beam::new((grid.len() as i32 - 1, i as i32), (-1, 0)))
        .collect();

    let mut most_e = 0;
    for start_beams in [left_col, right_col, top_row, bot_row] {
        for b in start_beams {
            let mut beams: Vec<Beam> = vec![b];
            let mut energized: HashSet<(i32, i32)> = HashSet::new();
            let mut cache: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

            while !beams.is_empty() {
                let visited = walk_beam(&mut beams, 0, &grid, &mut cache);
                for v in visited {
                    energized.insert(v);
                }
            }

            most_e = std::cmp::max(most_e, energized.len());
        }
    }

    println!("Part 2: {}", most_e);
}

fn p1(lines: Vec<String>) {
    let grid = lines_to_2d_chars(&lines);
    let mut beams: Vec<Beam> = vec![Beam::new((0, 0), (0, 1))];
    let mut energized: HashSet<(i32, i32)> = HashSet::new();
    let mut cache: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    while !beams.is_empty() {
        let visited = walk_beam(&mut beams, 0, &grid, &mut cache);
        for v in visited {
            energized.insert(v);
        }
    }

    println!("Part 1: {}", energized.len());
}

pub fn d16() {
    let input_file = Path::new("src/d16/in.txt");

    // p1(get_lines(input_file));
    p2(get_lines(input_file));
}
