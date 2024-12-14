use std::collections::HashMap;

use crate::util::{dump_grid, get_lines};

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(s: &str) -> Self {
        let (p, v) = s.split_once(" ").unwrap();

        let pv = p[2..]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let vv = v[2..]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Self {
            px: pv[0],
            py: pv[1],
            vx: vv[0],
            vy: vv[1],
        }
    }

    fn move_times(&mut self, times: i32, width: i32, height: i32) {
        self.px += self.vx * times;
        self.px = ((self.px % width) + width) % width;

        self.py += self.vy * times;
        self.py = ((self.py % height) + height) % height;
    }
}

fn sum_quadrant(robots: &[Robot], width: i32, height: i32) -> i32 {
    let middle_width = width / 2;
    let middle_height = height / 2;

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots {
        if robot.px == middle_width || robot.py == middle_height {
            continue;
        }

        match (robot.px < middle_width, robot.py < middle_height) {
            (true, true) => q1 += 1,
            (false, true) => q2 += 1,
            (true, false) => q3 += 1,
            (false, false) => q4 += 1,
        }
    }

    q1 * q2 * q3 * q4
}

fn display_robots(robots: &[Robot], width: i32, height: i32) {
    let mut grid = vec![vec!['.'; (width + 10) as usize]; (height + 10) as usize];

    for r in robots {
        grid[r.px as usize][r.py as usize] = 'X';
    }

    dump_grid(&grid);
}

fn p02(lines: Vec<String>) {
    let width = 101;
    let height = 103;

    let mut robots = lines.iter().map(|l| Robot::new(l)).collect::<Vec<_>>();

    // horiz funny patterns at i= 21, 122, 324
    // verti funny patterns at i= 97, 200, 406
    // horiz pattern at every base_h + width
    // verti pattern at every base_v + height
    let base_h = 21;
    let base_v = 97;

    for i in 0..1000000 {
        for robot in robots.iter_mut() {
            robot.move_times(1, width, height);
        }

        let e = (i - base_h) % width == 0;
        let d = (i - base_v) % height == 0;
        if e & d {
            println!("{i}");
            display_robots(&robots, width, height);
        }
        // if (i - base_h) % width == 0 || (i - base_v) % height == 0 {
        //     println!("{i}");
        //     display_robots(&robots, width, height);
        //     println!();
        //     println!();
        // }
    }
}

fn p01(lines: Vec<String>) {
    let width = 101;
    let height = 103;
    let mut robots = vec![];

    for line in lines {
        let mut robot = Robot::new(&line);
        robot.move_times(100, width, height);
        robots.push(robot);
    }

    let sum = sum_quadrant(&robots, width, height);
    println!("{}", sum);
}

pub fn d14() {
    p01(get_lines("src/d14/in.txt"));
    p02(get_lines("src/d14/in.txt"));
}
