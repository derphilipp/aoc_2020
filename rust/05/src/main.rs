//use std::ops::Range;
use std::path::Path;
use std::fs;

enum Move {
    Larger,
    Smaller
}


fn bn_search(low_inclusive: i32, high_exclusive:i32, movements: Vec<Move>) -> i32{
    let mut low = low_inclusive;
    let mut high = high_exclusive;

    let mut mid = ((high - low) / 2) + low;

    for movement in movements{
        match movement{
            Move::Larger => {
                low = mid;
            },
            Move::Smaller => {
                high = mid;
            },
        }
        mid = ((high - low) / 2) + low;
    }

    return mid;

}

fn parse_line(s: &str) -> Vec<Move>{
    let mut result: Vec<Move> = vec![];
    for c in s.chars(){
        match c{
            'B' => result.push(Move::Larger),
            'R' => result.push(Move::Larger),
            'F' => result.push(Move::Smaller),
            'L' => result.push(Move::Smaller),
            _=>{}
        }
    }
    return result;
}



fn both_ids(s: &str) -> (i32, i32){
    let first_part = &s[..7];
    let second_part = &s[7..];
    let first_id = bn_search(0,128, parse_line(first_part));
    let second_id = bn_search(0,8, parse_line(second_part));
    return( first_id, second_id);
}

fn full_id(s: & str) -> i32{
    let (i1, i2)=both_ids(s);
    i1*8+i2
}

fn part_one(){
    let data = fs::read_to_string(Path::new("input.txt")).unwrap();
    let maximum = data.trim_end().split("\n"). map(|x|full_id(x)).max().unwrap();
    println!("{:?}", maximum);
}

fn part_two(){
    let data = fs::read_to_string(Path::new("input.txt")).unwrap();
    let mut all_tickets:Vec<i32> = data.trim_end().split("\n"). map(|x|full_id(x)).collect();
    all_tickets.sort();
    let minimum_ticket = *all_tickets.first().unwrap(); // First row does not exist
    let maximum_ticket = *all_tickets.last().unwrap();

    let my_ticket:Vec<i32> = (minimum_ticket..maximum_ticket)
        .filter(|x| !all_tickets.contains(x))
        .collect();

    println!("{:?}", my_ticket);
}

fn main() {
    println!("Part one:");
    part_one();
    println!("Part two:");
    part_two();
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_0_in_7() {
        assert_eq!(bn_search(0,8,vec![Move::Smaller, Move::Smaller, Move::Smaller]) , 0);
    }
    #[test]
    fn find_7_in_7() {
        assert_eq!(bn_search(0,8,vec![Move::Larger, Move::Larger, Move::Larger]) , 7);
    }
    #[test]
    fn find_4_in_7() {
        assert_eq!(bn_search(0,8,vec![Move::Larger, Move::Smaller, Move::Smaller]) , 4);
    }
    #[test]
    fn find_44_in_127() {
        assert_eq!(bn_search(0,128,vec![
            Move::Smaller, Move::Larger, Move::Smaller, Move::Larger, Move::Larger,Move::Smaller, Move::Smaller,Move::Larger, Move::Smaller, Move::Larger,
        ]) , 44);
    }

    #[test]
    fn bfffbbf_means_70() {
        let f = parse_line("BFFFBBF");
        assert_eq!(bn_search(0,128,f) , 70);
    }
    #[test]
    fn fffbbbf_means_14() {
        let f = parse_line("FFFBBBF");
        assert_eq!(bn_search(0,128,f) , 14);
    }

    #[test]
    fn bbffbbf_means_102() {
        let f = parse_line("BBFFBBF");
        assert_eq!(bn_search(0,128,f) , 102);
    }


    #[test]
    fn bfffbbfrrr_means_70_7() {
        assert_eq!(both_ids("BFFFBBFRRR") , (70,7));
    }
    #[test]
    fn fffbbbfrrr_means_14_7() {
        assert_eq!(both_ids("FFFBBBFRRR") , (14,7));
    }
    #[test]
    fn bbffbbfrll_means_102_4() {
        assert_eq!(both_ids("BBFFBBFRLL") , (102,4));
    }


}