use crate::util::{dump_grid, get_lines};
use indexmap::IndexMap;

use std::cmp::Ordering;
use std::cmp::{max, min};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

type Space = Vec<Vec<Vec<char>>>;
type Bricks = IndexMap<char, Brick>;
type Dims = (usize, usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Brick {
    cubes: HashSet<[usize; 3]>,
    low_pos: usize,
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        let low_a = self.lowest_cube();
        let low_b = other.lowest_cube();

        low_a[2].cmp(&low_b[2])
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
        let mut cubes: HashSet<[usize; 3]> = HashSet::new();
        let mut low_pos = usize::MAX;

        for i in 0..3 {
            for j in 0..=(p2[i] - p1[i]) {
                let mut cube = [p1[0], p1[1], p1[2]];
                cube[i] += j;
                low_pos = min(low_pos, p1[2]);
                cubes.insert(cube);
            }
        }

        Brick { cubes, low_pos }
    }

    fn lowest_cube(&self) -> &[usize; 3] {
        self.cubes.iter().min_by(|&a, &b| a[2].cmp(&b[2])).unwrap()
    }

    fn lowest_cubes(&self) -> Vec<&[usize; 3]> {
        self.cubes
            .iter()
            .filter(|&c| c[2] == self.low_pos)
            .collect()
    }

    fn lower_brick(&mut self) {
        let mut new_cubes: HashSet<[usize; 3]> = HashSet::new();
        for cube in &self.cubes {
            let mut new_cube = cube.clone();
            new_cube[2] -= 1;
            new_cubes.insert(new_cube);
        }
        self.cubes = new_cubes;
    }
}

fn max_dims(bricks: &Bricks) -> Dims {
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

fn dump(space: &Space) {
    for x in 0..space.len() {
        for y in 0..space[x].len() {
            for z in 0..space[x][y].len() {
                print!("{}", space[x][y][z]);
            }
            println!();
        }
        println!("<<<<<<<<<<<<");
    }
}

enum ViewSide {
    X,
    Y,
}

fn view(space: &Space, side: ViewSide) {
    let mut view: Vec<Vec<char>> = vec![vec!['.'; space.len()]; space[0].len()];

    assert_eq!(space.len(), space[0][0].len());

    for x in 0..space.len() {
        for y in 0..space[x].len() {
            for z in 0..space[x][y].len() {
                let c = {
                    match side {
                        ViewSide::X => space[z][y][x],
                        ViewSide::Y => space[x][y][z],
                    }
                };
                if view[y][z] == '.' {
                    view[y][z] = c;
                }
            }
        }
    }

    dump_grid(&view);
}

fn make_space(lines: &Vec<String>) -> (Space, Bricks, Dims) {
    let mut bricks: Bricks = IndexMap::new();
    let names = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let mut ni: usize = 0;

    for line in lines {
        bricks.insert(names[ni], Brick::new(&line));
        ni += 1;
    }
    bricks.sort_by(|_, a, _, b| a.cmp(&b));

    let dims = max_dims(&bricks);

    let mut space: Space = vec![vec![vec!['.'; dims.2]; dims.1]; dims.0];

    for i in 0..dims.0 {
        for j in 0..dims.2 {
            space[i][dims.1 - 1][j] = '-';
        }
    }
    //
    for (name, brick) in &bricks {
        update_brick(&mut space, brick, &dims, *name);
    }

    (space, bricks, dims)
}

fn update_brick(space: &mut Space, brick: &Brick, dims: &Dims, name: char) {
    // todo: remove old cube
    for cube in &brick.cubes {
        space[cube[0]][dims.1 - 1 - cube[2]][cube[1]] = name;
    }
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let (mut space, bricks, dims) = make_space(&lines);
    view(&space, ViewSide::X);
    view(&space, ViewSide::Y);

    // Drop all bricks, starting at the lowest one
    for (name, mut brick) in bricks {
        let below_free = brick
            .lowest_cubes()
            .iter()
            .all(|&c| space[c[0]][dims.1 - 1 - c[2] + 1][c[1]] == '.');
        println!("{} {}", name, below_free);

        // TODO: drop brick until !below_free
        if below_free {
            brick.lower_brick();
            update_brick(&mut space, &brick, &dims, name);
            break;
        }
    }
    view(&space, ViewSide::X);
    view(&space, ViewSide::Y);
}

pub fn d22() {
    let input_file = Path::new("src/d22/dbg.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
