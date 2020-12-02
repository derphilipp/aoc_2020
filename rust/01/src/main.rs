use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.unwrap().parse::<i32>().expect("parse error"))
        .collect::<Vec<i32>>()
}

fn part_a(){
let lines = lines_from_file("input.txt");
for number1 in &lines {
    for number2 in &lines {
        if number1+number2 == 2020{
            let result = number1*number2;
            println!("{:?}", result);
            return
        }
    }
}
}

fn part_b(){
    let lines = lines_from_file("input.txt");
    for combination in lines.into_iter().combinations(3){
        let sum:i32=combination.iter().sum();
        if sum == 2020{
            let prod:i32 = combination.iter().product();
            println!("{:?}", prod);
            return;
        }
    }
}


fn main() {
    part_a();
    part_b();
}


