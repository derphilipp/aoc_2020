use std::path::Path;
use std::fs;
use std::str::FromStr;
use std::num::{ParseFloatError, ParseIntError};
use std::convert::TryFrom;


fn run(mut ins: Vec<Instruction>) -> (bool, i16) {
    let mut acc = 0;
    let mut line_nr=0;
    let target = ins.len();
    while line_nr<target && ins[line_nr].visited!=true   {
        ins[line_nr].visited=true;
        match ins[line_nr].command {
            Command::ACC =>  { acc+=ins[line_nr].value; line_nr+=1;},
            Command::JMP =>  {
                let nr=Number::try_from(line_nr).unwrap();
                line_nr =(nr + ins[line_nr].value) as usize;
            },
            Command::NOP =>  { line_nr+=1;},
       }
    }
    return (line_nr==target, acc);
}

fn part_one(){
    let instructions = load_instructions_from_file();
    let acc=run(instructions).1;
    println!("Final ACC: {:?}", acc);

}

fn part_two(){
    let instructions = load_instructions_from_file();

    for linenr in 0..instructions.len()-1 {
        match instructions[linenr].command {
        Command::NOP =>{
            let mut instructions_changeable = instructions.clone();
            instructions_changeable[linenr].command = Command::JMP;
            let result = run(instructions_changeable);
            if result.0 {
                println!("we got it! NOP=>JMP Line: {:?} ACC={:?}", linenr, result.1);
            }
        },
            Command::JMP => {
                let mut instructions_changeable = instructions.clone();
                instructions_changeable[linenr].command = Command::NOP;
                let result = run(instructions_changeable);
                if result.0 {
                    println!("we got it! JMP=>NOP Line: {:?} ACC={:?}", linenr, result.1);
                }
            },
            _=>{}
        }
    }
}


fn main() {
    println!("Part one:");
    part_one();
    println!("Part two:");
    part_two();


}

type Number =i16;

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



#[derive(Debug, PartialEq, Clone)]
pub enum Command{
    NOP,
    ACC,
    JMP
}

#[derive(Clone)]
struct Instruction{
    command: Command,
    value: Number,
    visited: bool,
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(line: &str) -> Result<Self, ParseError>  {
        let  (mut str_first, mut str_second) = line.split_at(3);
        str_first = str_first.trim();
        str_second=str_second.trim();
        let command = match str_first{
            "acc" => Command::ACC,
            "jmp" => Command::JMP,
            "nop" => Command::NOP,
            _ => return Err(ParseError::Malformed("empty first body component")),
        };
        let value= str_second.parse::<Number>().unwrap();
        Ok(Instruction{command: command, value:value, visited:false})
    }

}

#[allow(dead_code)]
fn load_instructions_from_file() -> Vec<Instruction> {
    let data = fs::read_to_string(Path::new("input.txt")).unwrap();
    load_instructions(&data)
}


fn load_instructions(data: &str)->Vec<Instruction>{
    let mut instructions:Vec<Instruction>=Vec::new();
    for line in data.split("\n"){
        let l=Instruction::from_str(line).unwrap();
        instructions.push(l);
    }
    instructions
}
#[allow(dead_code)]
fn load_simple_instructions() -> Vec<Instruction>{
    load_instructions(r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_instructions() {
        let i = load_instructions(r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#);
        assert_eq!(i.len(), 9);
        assert_eq!(i[0].command,Command::NOP);
        assert_eq!(i[5].value,-99);
    }
}