use std::fs;
use std::path::Path;

fn main() {
    println!("Part one");
    part_one();
    println!("Part two");
    part_two();
}

fn find_prods(data: &Vec<i64>, value_to_find: i64) -> (usize, usize) {
    for f_from in 0..data.len() {
        for f_to in f_from + 1..data.len() {
            let hit = data[f_from..f_to].iter().sum::<i64>();
            if hit == value_to_find {
                return (f_from, f_to);
            }
        }
    }
    (0, 0)
}

fn find_mults(data: &Vec<i64>, length: usize, position: usize) -> (i64, i64) {
    let value_to_find = data[position];
    for outer in (position - length)..(position) {
        for inner in (position - length)..(position) {
            if inner != outer {
                if data[inner] + data[outer] == value_to_find {
                    return (inner as i64, outer as i64);
                }
            }
        }
    }
    (-1, -1)
}

fn first_invalid_position(nrs: &Vec<i64>, length: usize) -> usize {
    for r in length..nrs.len() {
        let x = find_mults(&nrs, length, r);
        if x == (-1, -1) {
            return r;
        }
    }
    return 0;
}

fn part_one() {
    let nrs = load_real_numbers();
    let i = first_invalid_position(&nrs, 25);
    println!("First invalid position: {:?}, value: {:?}", i, nrs[i]);
}

fn part_two() {
    let nrs = load_real_numbers();
    let i = first_invalid_position(&nrs, 25);
    let (a, b) = find_prods(&nrs, nrs[i]);
    let mx = nrs[a..b].iter().max().unwrap();
    let mi = nrs[a..b].iter().min().unwrap();
    for x in nrs[a..b].iter() {
        println!("{:?}", x);
    }
    println!("Max {:?}, Min {:?}, Sum {:?}", mx, mi, mx + mi);
}

fn load_real_numbers() -> Vec<i64> {
    let data = fs::read_to_string(Path::new("input.txt")).unwrap();
    let x = data.as_str();
    load_numbers(&x)
}

fn load_simple_numbers() -> Vec<i64> {
    let data = fs::read_to_string(Path::new("input_sample.txt")).unwrap();
    let x = data.as_str();
    load_numbers(&x)
}

#[allow(dead_code)]
fn load_numbers(data: &str) -> Vec<i64> {
    data.split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}
