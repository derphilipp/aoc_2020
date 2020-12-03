use std::{
    fs::File,
    io::{prelude::*, BufReader},
    iter::Iterator,
};

#[derive(Debug)]
struct MapLine {
    trees_in_line: Vec<usize>,
    width: usize,
}

#[derive(Debug)]
struct Strategy {
    increase_horizontal: usize,
    increase_vertical: usize,
}

fn line_to_mapline(line: String) -> MapLine {
    MapLine {
        trees_in_line: line
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '#')
            .map(|(i, _)| i)
            .collect(),
        width: line.len(),
    }
}

fn part_one() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open file.txt"));
    let mut amount_of_trees: usize = 0;
    let mut horizontal_position: usize = 0;

    for line in reader.lines() {
        let p = line_to_mapline(line.unwrap());
        let current_line = horizontal_position % p.width;
        if p.trees_in_line.contains(&current_line) {
            amount_of_trees += 1;
        }
        horizontal_position += 3;
    }
    println!("Amount of trees: {}", amount_of_trees);
}

fn data() -> Vec<MapLine> {
    let data: Vec<MapLine> = BufReader::new(File::open("input.txt").expect("Cannot open file.txt"))
        .lines()
        .map(|l| line_to_mapline(l.unwrap()))
        .collect();
    return data;
}

fn part_two() {
    let strategies = vec![
        Strategy {
            increase_vertical: 1,
            increase_horizontal: 1,
        },
        Strategy {
            increase_vertical: 1,
            increase_horizontal: 3,
        },
        Strategy {
            increase_vertical: 1,
            increase_horizontal: 5,
        },
        Strategy {
            increase_vertical: 1,
            increase_horizontal: 7,
        },
        Strategy {
            increase_vertical: 2,
            increase_horizontal: 1,
        },
    ];

    let counter: usize = strategies
        .iter()
        .map(|strategy| {
            data()
                .iter()
                .step_by(strategy.increase_vertical)
                .zip((0..).step_by(strategy.increase_horizontal))
                .filter(|(x, i)| x.trees_in_line.contains(&(i % x.width)))
                .count()
        })
        .collect::<Vec<_>>()
        .iter()
        .product();

    println!("Result: {}", counter);
}

fn main() {
    println!("Part one");
    part_one();
    println!("Part two");
    part_two();
}
