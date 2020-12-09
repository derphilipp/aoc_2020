use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;
use std::str::FromStr;

use lazy_static::lazy_static;
use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use regex::Regex;

type Amount = usize;
type Bag = String;
type BagsTo = HashMap<Bag, Amount>;

fn main() {
    println!("Part one");
    part_one();
    println!("Part two");
    part_two();
}

#[allow(dead_code)]
fn load_simple_rules() -> Vec<BagRule> {
    let rules = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
    let global_bag_rules = load_rules(rules);
    return global_bag_rules;
}

fn part_two() {
    let input_text = fs::read_to_string(Path::new("input.txt")).unwrap();
    let input_text_str = input_text.as_str();

    let a = load_rules_graph(input_text_str);
    let b = contains_bag(&a, "shiny gold");
    println!("Amount of bags required: {:?}", b);
}


fn part_one() -> usize {
    let global_bag_rules = load_rules_from_file();

    let target_bag: Bag = Bag::from("shiny gold");
    let mut reachable_bags: HashSet<Bag> = HashSet::new();
    reachable_bags.insert(target_bag);

    let mut amount_last_time = 0;
    let mut amount_this_time = 1;

    while amount_last_time != amount_this_time {
        amount_last_time = amount_this_time;
        let mut bags_to_add: HashSet<Bag> = HashSet::new();

        for global_bag_rule in &global_bag_rules {
            for reachable_bag in &reachable_bags {
                if global_bag_rule.bags_to.contains_key(reachable_bag) {
                    let b = Bag::from(global_bag_rule.bag_from.clone());
                    bags_to_add.insert(b);
                }
            }
        }
        bags_to_add.iter().for_each(|x| {
            reachable_bags.insert(x.clone());
        });
        amount_this_time = reachable_bags.len();
    }
//    println!("{:?}",reachable_bags);
    println!("Amount rules: {:?}, can reach: {:?}", global_bag_rules.len(), reachable_bags.len());
    println!("Answer therefore is {:?}", reachable_bags.len() - 1);
    reachable_bags.len() - 1
}


fn load_rules_nodes(data: &str) -> HashSet<Bag> {
    let mut all_bags: HashSet<Bag> = HashSet::new();

    for line in data.split("\n") {
        let l = BagRule::from_str(line).unwrap();
        for target in l.bags_to {
            all_bags.insert(target.0);
        }
        all_bags.insert(l.bag_from);
    }
    all_bags
}

fn load_rules_graph(data: &str) -> DiGraph<Bag, Amount> {
    let mut graph = DiGraph::<Bag, Amount>::new();
    let rules = load_rules(data);
    let bags_as_vec = load_rules_nodes(data).into_iter().collect::<Vec<Bag>>();

    let mut elements_in_graph: HashMap<Bag, _> = HashMap::new();

    for x in &bags_as_vec {
        elements_in_graph.insert(x.clone(), graph.add_node(x.clone()));
    }

    for r in rules {
        for t in r.bags_to {
            graph.add_edge(elements_in_graph[&r.bag_from], elements_in_graph[&t.0], t.1);
        }
    }
    graph
}

fn edge_counts(graph: &DiGraph<String, usize>, parent: NodeIndex, node: NodeIndex) -> usize {
    let bag_count_edge = graph.find_edge(parent, node).unwrap();
    let bag_count = *(graph.edge_weight(bag_count_edge).unwrap());
    let neighbors = graph.neighbors_directed(node, Direction::Outgoing);
    let nested_count: usize = if neighbors.count() == 0 {
        0
    } else {
        graph.neighbors_directed(node, Direction::Outgoing).map(|n| bag_count * edge_counts(graph, node, n)).sum()
    };
    bag_count + nested_count
}

fn contains_bag(input: &DiGraph<Bag, Amount>, search_for: &str) -> usize {
    let flip = input.clone();
    let shiny_gold_index = flip
        .node_indices()
        .find(|i| input[*i] == search_for)
        .unwrap();

    input
        .neighbors_directed(shiny_gold_index, Direction::Outgoing)
        .map(|node| edge_counts(input, shiny_gold_index, node))
        .sum()
}

fn load_rules(data: &str) -> Vec<BagRule> {
    let mut rules: Vec<BagRule> = Vec::new();
    for line in data.split("\n") {
        let l = BagRule::from_str(line).unwrap();
        rules.push(l);
    }
    rules
}

fn load_rules_from_file() -> Vec<BagRule> {
    let data = fs::read_to_string(Path::new("input.txt")).unwrap();
    load_rules(&data)
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


#[derive(Debug)]
struct RuleDestiny {
    bag: Bag,
    amount: Amount,
}

#[derive(Debug)]
struct BagRule {
    bag_from: Bag,
    bags_to: BagsTo,
}

impl FromStr for BagRule {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, ParseError> {
        lazy_static! {
          static ref RE_FINISHING_RULE: Regex = Regex::new(r"^(?P<from>.+) bags contain no other bags\.$").unwrap();
          static ref RE_RULES: Regex = Regex::new(r"^(?P<from>.+) bags contain (?P<rules_to>.+)\.$").unwrap();
          // light red bags contain '1 bright white bag, 2 muted yellow bags'.
          static ref RE_RULE: Regex = Regex::new(r"[ ]*(?P<amount>\d+) (?P<bag>.+?) bags?[ ]*").unwrap();
         }

        let result: BagRule;
        if RE_FINISHING_RULE.is_match(line) {
            let caps = RE_FINISHING_RULE.captures(line).ok_or_else(|| ParseError::Malformed("empty first body component"))?;
            let bag_from: Bag = Bag::from(caps.name("from").unwrap().as_str());
            let bags_to: BagsTo = BagsTo::new();
            result = BagRule { bag_from, bags_to };
        } else {
            let caps = RE_RULES.captures(line).ok_or_else(|| ParseError::Malformed("empty first body component"))?;
            let bag_from = Bag::from(caps.name("from").unwrap().as_str());
            let mut bags_to: BagsTo = BagsTo::new();
            let bags_to_text = Bag::from(caps.name("rules_to").unwrap().as_str());
            for bag_line in bags_to_text.split(",") {
                let caps = RE_RULE.captures(bag_line).ok_or_else(|| ParseError::Malformed("empty first body component"))?;
                let bag_to: Bag = Bag::from(caps.name("bag").unwrap().as_str());
                let bag_amount = caps.name("amount").unwrap().as_str().parse::<Amount>().unwrap();
                bags_to.insert(bag_to, bag_amount);
            }
            result = BagRule { bag_from, bags_to };
        }
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bag_rule() {
        let br = BagRule::from_str("dotted black bags contain no other bags.").unwrap();
        assert_eq!(br.bag_from.as_str(), "dotted black");
        assert_eq!(br.bags_to.len(), 0);
    }

    #[test]
    fn example_bag_rule() {
        let br = BagRule::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap();
        assert_eq!(br.bag_from.as_str(), "light red");
        assert_eq!(br.bags_to.len(), 2);
        assert_eq!(br.bags_to["muted yellow"], 2);
    }

    #[test]
    fn part_two_parse_text() {
        let rules = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let parsed_rules = load_rules(rules);
        assert_eq!(parsed_rules.len(), 9);
    }

    #[test]
    fn part_one_is_correct() {
        assert_eq!(part_one(), 272);
    }
}
