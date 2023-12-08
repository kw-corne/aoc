use indexmap::IndexMap;
use std::{collections::HashMap, path::Path};

use crate::util::get_lines;

#[derive(Debug)]
struct Graph {
    nodes: IndexMap<String, (String, String)>,
    instr: Vec<char>,
}

impl Graph {
    fn from_lines(lines: &Vec<String>) -> Self {
        let mut instr: Vec<char> = vec![];
        for c in lines[0].chars() {
            instr.push(c);
        }

        let mut nodes: IndexMap<String, (String, String)> = IndexMap::new();
        for line in lines.iter().skip(2) {
            let words: Vec<&str> = line.split_whitespace().collect();

            let mut l = words[2].to_string();
            l.remove(0);
            l.remove(l.len() - 1);

            let mut r = words[3].to_string();
            r.remove(l.len());

            nodes.insert(words[0].to_string(), (l, r));
        }

        Self { nodes, instr }
    }
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let graph = Graph::from_lines(&lines);

    let last = "ZZZ".to_string();
    let mut curr = &"AAA".to_string();
    let mut iters = 0;

    while curr != "ZZZ" {
        for dir in &graph.instr {
            let next = &graph.nodes.get(curr).unwrap();
            if *dir == 'L' {
                curr = &next.0;
            } else {
                curr = &next.1;
            }

            iters += 1;
            if *curr == last {
                break;
            }
        }
    }

    println!("Part 1: {}", iters);
}

pub fn d08() {
    let input_file = Path::new("src/d08/in.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
