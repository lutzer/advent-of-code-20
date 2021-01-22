use std::fs;
use clap::{Arg, App};

const FILENAME : &str = "input.txt";

macro_rules! str {
  ($e:expr) => { String::from($e) };
}

type ContainedBag = (i32, String);

struct Rule {
  bag: String,
  contains: Vec<ContainedBag> // (count, bag)
}

impl Rule {
  #[allow(dead_code)]
  fn print(&self) {
    println!("\t{}:{:?}", self.bag, self.contains);
  }
}

fn parse_rule(data : &str) -> Rule {
  let parts : Vec<&str> = data.split(" bags contain ").collect();
  let mut contains : Vec<ContainedBag> = vec![];
  for bag_string in parts[1].split(", ") {
    let bag_string_parts : Vec<&str> = bag_string.split(" ").collect();
    if bag_string_parts[0] != "no" {
      let count = bag_string_parts[0].parse().unwrap_or(0);
      let name = [bag_string_parts[1], bag_string_parts[2]].join(" ");
      contains.push((count, str!(&name)));
    }
  };
  return Rule { bag: str!(parts[0]), contains: contains }
}

fn contains_shiny_gold_bag(bag : String, rules: &Vec<Rule>) -> bool {
  let rule = rules.into_iter().find(|r| { r.bag == bag }).unwrap();

  let contains = rule.contains.clone();
  for contained_bag in contains {
    if contained_bag.1 == "shiny gold" {
      return true
    } else { 
      let contains_check = contains_shiny_gold_bag( contained_bag.1, rules);
      if contains_check { return true }
    }
  }

  // if empty return false
  return false;
}

fn count_containing_bags(bag: String, rules: &Vec<Rule>) -> i32 {
  let rule = rules.into_iter().find(|r| { r.bag == bag }).unwrap();

  let contains = rule.contains.clone();

  if contains.len() == 0 {
    return 0;
  } else {
    return contains.into_iter().fold(0, |acc, c| {
      acc + c.0 + c.0 * count_containing_bags(c.1, rules)
    });
  }
}

fn part_1() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let lines = data.split("\n");

  let rules : Vec<Rule> = lines.map(|l| { parse_rule(l) }).collect();

  let contains = (&rules).into_iter().map(|rule| {
    let bag = rule.bag.clone();
    let contains = contains_shiny_gold_bag(bag.clone(), &rules);
    (bag, contains)
  });

  let sum = contains.filter(|c| { c.1 == true }).count();

  println!("sum of bags containing shiny gold bag: {}", sum);
}

fn part_2() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let lines = data.split("\n");

  let rules : Vec<Rule> = lines.map(|l| { parse_rule(l) }).collect();

  let count = count_containing_bags(str!("shiny gold"), &rules);

  println!("one shiny gold bag contains {} other bags.", count);
}

fn main() {
  let args = App::new("Advent of Code - Day 7")
  .arg(Arg::with_name("day")
  .takes_value(true)
  .required(true))
  .get_matches();
  
  let day = args.value_of("day").unwrap_or("");
  
  match day {
    "1" => part_1(),
    "2" => part_2(),
    _ =>  eprintln!("Select either part 1 or 2")
  }
}