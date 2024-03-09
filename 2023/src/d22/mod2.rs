use indexmap::IndexSet;
use ndarray::{s, Array3, AssignElem, Axis, Zip};
use std::cmp::max;
use std::collections::HashMap;
use std::path::Path;

use crate::util::get_lines;

#[derive(Debug)]
struct Space {
    arr: Array3<char>,
}

impl Space {
    fn new(bricks: &HashMap<char, Brick>) -> Self {
        let dims = Space::max_dims(&bricks);
        let mut arr = Array3::<char>::from_elem(dims, '.');

        let mut floor = arr.slice_mut(s![.., dims.1 - 1, ..]);
        floor.fill('-');

        for (name, brick) in bricks {
            for cube in &brick.cubes {
                arr[[cube[0], dims.1 - 1 - cube[2], cube[1]]] = *name;
            }
        }

        Self { arr }
    }

    fn view_x(&self) {}

    fn view_y(&self) {}

    fn max_dims(bricks: &HashMap<char, Brick>) -> (usize, usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;

        for brick in bricks.values() {
            for cube in &brick.cubes {
                max_x = max(max_x, cube[0]);
                max_y = max(max_y, cube[1]);
                max_z = max(max_z, cube[2]);
            }
        }

        (max_x + 1, max_z + 1, max_y + 1)
    }
}

#[derive(Debug)]
struct Brick {
    cubes: IndexSet<[usize; 3]>,
}

impl Brick {
    fn new(line: &str) -> Self {
        let sp = line.split_once('~').unwrap();
        let p1: Vec<usize> =
            sp.0.split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
        let p2: Vec<usize> =
            sp.1.split(',')
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

        // assuming input has property: p1 < p2
        // assuming only one of x,y,z increases
        let mut cubes: IndexSet<[usize; 3]> = IndexSet::new();

        for i in 0..3 {
            for j in 0..=(p2[i] - p1[i]) {
                let mut cube = [p1[0], p1[1], p1[2]];
                cube[i] += j;
                cubes.insert(cube);
            }
        }

        Brick { cubes }
    }
}

fn get_bricks(lines: &Vec<String>) -> HashMap<char, Brick> {
    let mut bricks: HashMap<char, Brick> = HashMap::new();
    let names = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let mut ni: usize = 0;

    for line in lines {
        let b = Brick::new(&line);
        bricks.insert(names[ni], b);
        ni += 1;
    }

    bricks
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let bricks = get_bricks(&lines);
    let mut space = Space::new(&bricks);
    space.view_x();
}

pub fn d22() {
    let input_file = Path::new("src/d22/dbg.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
