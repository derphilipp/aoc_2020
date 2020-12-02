use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use regex::Regex;

struct PasswordLine {
    password: String,
    letter: char,
    amount_from: usize,
    amount_to: usize,
}

fn line_to_password(line: String) -> PasswordLine{
    let re = Regex::new(r"^(?P<amount_from>[\d]+)-(?P<amount_to>[\d]+) (?P<letter>[A-Za-z]): (?P<password>.+)$").unwrap();
    let caps=re.captures(line.as_str()).unwrap();
    PasswordLine {
        amount_from: caps["amount_from"].parse::<usize>().expect("parse error"),
        amount_to: caps["amount_to"].parse::<usize>().expect("parse error"),
        letter: caps["letter"].parse::<char>().expect("parse error"),
        password: caps["password"].parse::<String>().expect("parse error")
    }
}

fn is_valid_part_one(p: PasswordLine) -> bool{
    (p.amount_from .. p.amount_to+1).contains(&p.password.matches(p.letter).count())
}

fn is_valid_part_two(p: &PasswordLine) -> bool{
    (p.password.chars().nth(p.amount_from-1).unwrap() == p.letter) ^ (p.password.chars().nth(p.amount_to-1).unwrap() == p.letter)
}

fn part_one(){
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open file.txt"));
    let mut i=0;
    // Part 1
    for line in reader.lines(){
        let p=line_to_password(line.unwrap());
        if is_valid_part_one(p){
            i=i+1;
        }
    }
    println!("{:?}",i);
}

fn part_two_a(){
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open file.txt"));
    let mut i=0;
    for line in reader.lines(){
        let p=line_to_password(line.unwrap());
        if is_valid_part_two(&p){
            i=i+1;
        }
    }
    println!("{:?}",i);
}

fn part_two_b(){
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open file.txt"));
    let amount=reader.lines()
        .map(|l| line_to_password(l.unwrap()))
        .filter(|l| is_valid_part_two(l))
        .count() ;
    println!("{:?}",amount);
}

fn main(){
    part_one();
    part_two_a();
    part_two_b();
}