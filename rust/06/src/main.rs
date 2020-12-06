use std::path::Path;
use std::fs;
use std::collections::{HashSet};
//use std::iter::FromIterator;

fn main() {
    println!("Part one");
    part_one();
    println!("Part two");
    part_two();
}

fn part_one(){
    let data = fs::read_to_string(Path::new("input.txt")).unwrap();
    let result = parse_blocks_a(data);
    println!("Result: {:?}", result);
}

fn part_two(){
    let data = fs::read_to_string(Path::new("input.txt")).unwrap();
    let result = parse_blocks_b(data);
    println!("Result: {:?}", result);
}


fn parse_blocks_b(s:String) -> usize{
    let spl = s.trim().split("\n\n");
    spl.map(|x| parse_group_b(x)).sum()
}

fn parse_blocks_a(s:String) -> usize{
    let spl = s.trim().split("\n\n");
    spl.map(|x| parse_group_a(x)).sum()
}


fn parse_group_a(s:&str) -> usize{
    let mut char_vec:Vec<char> = s.replace("\n","").chars().collect();
    char_vec.sort();
    char_vec.dedup();
    char_vec.len()
}

fn parse_group_b(s:&str) -> usize{
    let lines:Vec<&str> = s.split("\n").collect();

    let mut lines_as_vec:Vec<HashSet<char>>= [].iter().cloned().collect();
    for l in lines{
        lines_as_vec.push(l.chars().into_iter().collect());
    }

    let mut first:HashSet<char> = lines_as_vec.first().unwrap().clone();
    for l in &lines_as_vec{
        first = first.intersection(&l).cloned().collect();
    }
    first.len()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_a_means_one() {
        assert_eq!(parse_blocks_a(String::from("a\n")), 1);
    }

    #[test]
    fn part_1_abc_means_three() {
        assert_eq!(parse_blocks_a(String::from("abc\n")), 3);
    }

    #[test]
    fn part_1_a_b_c_means_three() {
        assert_eq!(parse_blocks_a(String::from("a\nb\nc\n")), 3);
    }

    #[test]
    fn part_1_ab_ac_means_three(){
        assert_eq!(parse_blocks_a(String::from("ab\nac\n")), 3);
    }

    #[test]
    fn part_1_a_a_a_a_means_one(){
        assert_eq!(parse_blocks_a(String::from("a\na\na\na\n")), 1);
    }

    #[test]
    fn part_1_b_means_one(){
        assert_eq!(parse_blocks_a(String::from("b")), 1);
    }

    #[test]
    fn part_1_multiline_parse() {
        let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
"#;
        assert_eq!(parse_blocks_a(String::from(input)), 11);
    }


    #[test]
    fn part_2_a_means_one() {
        assert_eq!(parse_blocks_b(String::from("a\n")), 1);
    }

    #[test]
    fn part_2_abc_means_three() {
        assert_eq!(parse_blocks_b(String::from("abc\n")), 3);
    }

    #[test]
    fn part_2_a_b_c_means_zero() {
        assert_eq!(parse_blocks_b(String::from("a\nb\nc\n")), 0);
    }

    #[test]
    fn part_2_ab_ac_means_one(){
        assert_eq!(parse_blocks_b(String::from("ab\nac\n")), 1);
    }

    #[test]
    fn part_2_a_a_a_a_means_one(){
        assert_eq!(parse_blocks_b(String::from("a\na\na\na\n")), 1);
    }

    #[test]
    fn part_2_b_means_one(){
        assert_eq!(parse_blocks_b(String::from("b")), 1);
    }

    #[test]
    fn part_2_multiline_parse() {
        let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
"#;
        assert_eq!(parse_blocks_b(String::from(input)), 6);
    }


}