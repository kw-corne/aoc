use std::{collections::HashSet, path::Path};

use crate::util::{dump_grid, dump_hash_set_points, get_lines, lines_to_2d_chars};

type Point = [usize; 2];

fn get_points_and_folds(lines: &Vec<String>) -> (HashSet<Point>, Vec<(char, usize)>) {
    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<(char, usize)> = vec![];

    let mut i = 0;
    while !lines[i].is_empty() {
        let (x, y) = lines[i].split_once(',').unwrap();
        points.insert([x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()]);
        i += 1;
    }

    i += 1;

    while i < lines.len() {
        let fold_instr: &str = lines[i].split_whitespace().skip(2).collect::<Vec<_>>()[0];
        let (axis, num) = fold_instr.split_once('=').unwrap();

        folds.push((axis.chars().next().unwrap(), num.parse::<usize>().unwrap()));

        i += 1;
    }

    (points, folds)
}

fn p2(lines: Vec<String>) {
    let (mut curr_points, folds) = get_points_and_folds(&lines);

    for fold in folds {
        let fold_axis = match fold.0 {
            'x' => 0,
            'y' => 1,
            _ => panic!(),
        };

        let mut new_points: HashSet<Point> = HashSet::new();

        for point in &curr_points {
            if point[fold_axis] < fold.1 {
                new_points.insert(point.clone());
            } else {
                let mut new_point = point.clone();
                new_point[fold_axis] = fold.1 - (point[fold_axis] - fold.1);
                new_points.insert(new_point);
            }
        }

        curr_points = new_points;
    }

    dump_hash_set_points(&curr_points);
}

fn p1(lines: Vec<String>) {
    let (points, folds) = get_points_and_folds(&lines);

    let fold_axis = match folds[0] {
        ('x', _) => 0,
        ('y', _) => 1,
        _ => panic!(),
    };

    let mut new_points: HashSet<Point> = HashSet::new();
    for point in &points {
        if point[fold_axis] < folds[0].1 {
            new_points.insert(point.clone());
        } else {
            let mut new_point = point.clone();
            new_point[fold_axis] = folds[0].1 - (point[fold_axis] - folds[0].1);
            new_points.insert(new_point);
        }
    }

    println!("{}", new_points.len());
}

pub fn d13() {
    let input_file = Path::new("src/d13/in.txt");

    p1(get_lines(input_file));
    p2(get_lines(input_file));
}
