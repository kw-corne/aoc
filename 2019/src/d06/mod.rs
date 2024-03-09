use core::panic;
use std::{collections::HashMap, path::Path};

use crate::util::get_lines;

fn get_indexes_of_first_duplicate<T>(v1: Vec<T>, v2: Vec<T>) -> Option<(usize, usize)>
where
    T: Eq + PartialEq,
{
    for i in 0..v1.len() {
        for j in 0..v2.len() {
            if v1[i] == v2[j] {
                return Some((i, j));
            }
        }
    }

    None
}

struct OrbitMap<'a>(HashMap<&'a str, &'a str>);

impl<'a> OrbitMap<'a> {
    fn from_aoc_input(lines: &'a Vec<String>) -> Self {
        let mut orbit_map: HashMap<&str, &str> = HashMap::new();

        for line in lines {
            let (inner, outer) = line.split_once(")").unwrap();

            if let Some(e) = orbit_map.insert(outer, inner) {
                panic!("Orbiting more than 1: {}", e);
            }
        }

        Self { 0: orbit_map }
    }

    fn count_orbits(&self, object: &str) -> i32 {
        let mut count = 0;
        let mut curr_object = object;

        while let Some(orbits) = self.0.get(curr_object) {
            count += 1;
            curr_object = orbits;
        }

        count
    }

    fn total_orbits(&self) -> i32 {
        let xd = 10;
        self.0.keys().map(|k| self.count_orbits(k)).sum()
    }

    fn path_to_com(&self, start_object: &'a str) -> Vec<&'a str> {
        let mut curr_object = start_object;
        let mut path: Vec<&str> = vec![];

        while curr_object != "COM" {
            curr_object = self.0[curr_object];
            path.push(curr_object);
        }

        path
    }

    fn transfers_between(&self, object1: &str, object2: &str) -> i32 {
        let path_to_com_from = self.path_to_com(object1);
        let path_to_com_to = self.path_to_com(object2);

        let (idx_from, idx_to) =
            get_indexes_of_first_duplicate(path_to_com_from, path_to_com_to).unwrap();

        (idx_from + idx_to) as i32
    }
}

fn p2(lines: Vec<String>) {
    let orbit_map = OrbitMap::from_aoc_input(&lines);
    let distance_you_san = orbit_map.transfers_between("YOU", "SAN");

    println!("{}", distance_you_san);
}

fn p1(lines: Vec<String>) {
    let orbit_map = OrbitMap::from_aoc_input(&lines);
    let sum = orbit_map.total_orbits();

    println!("{}", sum);
}

pub fn d06() {
    let input_file = Path::new("src/d06/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
