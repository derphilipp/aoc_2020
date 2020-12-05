
use std::{
    fs,
    path::Path,
    str::FromStr,
    num::{ParseFloatError, ParseIntError},

};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Size {
    name: String,
    size:i32
}

pub enum ParseFieldsError {
    /// The provided field is not known.
    UnknownField,
}



#[derive(Debug, PartialEq)]
pub enum ParseError {
    Malformed(&'static str),
    UnknownMetricType(String),
    InvalidInteger(ParseIntError),
    InvalidFloat(ParseFloatError),
}


impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> ParseError {
        ParseError::InvalidInteger(e)
    }
}

fn valid_hcl(line: &String) -> bool{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
    return RE.is_match(&line);
}

fn valid_ecl(line: &String) -> bool{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }
    return RE.is_match(&line);
}
fn valid_pid(line: &String) -> bool{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
    return RE.is_match(&line);
}

impl FromStr for Size {
   type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, ParseError> {
        lazy_static! {

        static ref RE: Regex = Regex::new(r"^(?P<number>[\d]{2,3})(?P<unit>in|cm)$").unwrap();
        }
        let caps = RE.captures(line).ok_or_else(|| ParseError::Malformed("empty first body component"))?;
        let n:String = String::from_str(caps.name("unit").unwrap().as_str()).unwrap();
        let s:i32 = caps.name("number").unwrap().as_str().parse::<i32>()?;
        let result = Size{ name:n,size:s};
        Ok(result)
        }
    }


#[derive(Debug)]
struct Passport {
    byr:Option<i32>,
    iyr:Option<i32>,
    eyr:Option<i32>,
    hgt:Option<Size>,
    hcl:Option<String>,
    ecl:Option<String>,
    pid:Option<String>,
    cid:Option<i32>,
}

fn read_passports(filename: &str) -> Vec<Passport> {
    let mut result:Vec<Passport>=Vec::new();
    let data =fs::read_to_string(Path::new(filename)).unwrap();
    data
      .trim_end()
      .split("\n\n")
      .map(|x|x.replace('\n'," "))
      .for_each(|x| {
          let mut p:Passport = Passport{byr:None, iyr:None, eyr:None, hgt:None, hcl:None,ecl:None, pid:None, cid:None};
          x.split(" ").for_each(|pair| {
              let pr =pair.split(":").collect::<Vec<&str>>();
              let mut pri = pr.iter();
              let first = pri.next().unwrap();
              let second = pri.next().unwrap();
              match first{
                  &"byr" => p.byr=Some(second.parse::<i32>().unwrap()),
                  &"iyr" => p.iyr=Some(second.parse::<i32>().unwrap()),
                  &"eyr" => p.eyr=Some(second.parse::<i32>().unwrap()),
                  &"hgt" => {
                            let data= second.parse::<Size>();
                             if data.is_ok(){
                                p.hgt=Some(data.unwrap());
                             }
                            },
                  &"hcl" => p.hcl=Some(second.to_string()),
                  &"ecl" => p.ecl=Some(second.to_string()),
                  &"pid" => p.pid=Some(second.to_string()),
                  &"cid" => p.cid=Some(second.parse::<i32>().unwrap()),
                  _ => {println!("Something else")}
              };
          });
          result.push(p);
      });
      return result;
}

fn part_one() {
    let passports=read_passports("input.txt");
    let mut i=0;
    let mut amount=0;
    for p in passports {
        if !(
            p.byr.is_none() ||
            p.iyr.is_none() ||
            p.eyr.is_none() ||
            p.hgt.is_none() ||
            p.hcl.is_none() ||
            p.ecl.is_none() ||
            p.pid.is_none()
        ){
        i=i+1;
    }
        amount+=1;
    }
    println!("Counted: {:?}",amount);
    println!("Valid: {:?}",i);
}

fn valid_hgt(s: &Size) -> bool{
    return if s.name == "in" {
        s.size >= 59 && s.size <= 76
    } else {
        s.size >= 150 && s.size <= 193
    }

}

fn part_two() {
    let passports=read_passports("input.txt");

    let i=passports.iter()
        .filter(|x|x.byr.is_some())
        .filter(|x|x.iyr.is_some())
        .filter(|x|x.eyr.is_some())
        .filter(|x|x.hgt.is_some())
        .filter(|x|x.hcl.is_some())
        .filter(|x|x.ecl.is_some())
        .filter(|x|x.pid.is_some())
        .filter(|x|x.byr.unwrap() >=1920 && x.byr.unwrap() <=2002)
        .filter(|x|x.iyr.unwrap() >=2010 && x.iyr.unwrap() <=2020)
        .filter(|x|x.eyr.unwrap() >=2020 && x.eyr.unwrap() <=2030)
        .filter(|x|valid_hcl(x.hcl.as_ref().unwrap()))
        .filter(|x|valid_ecl(x.ecl.as_ref().unwrap()))
        .filter(|x|valid_pid(x.pid.as_ref().unwrap()))
        .filter(|x|valid_hgt(x.hgt.as_ref().unwrap()))
        .count();
    println!("Valid: {:?}", i);
}

fn main() {
    println!("Part one");
    part_one();
    println!("Part two");
    part_two();
}
