use std::fs;
use clap::{Arg, App};
use std::collections::HashMap;
use str_macro::str;

const FILENAME: &str = "input.txt";

type RuleMap = std::collections::HashMap<String, std::vec::Vec<String>>;



fn match_rule(message: &str, rule_key: String, rule_map: &RuleMap) -> (bool, usize) {
  let rules = rule_map.get(&rule_key[..]).expect("Did not find rule");
  let mut steps = 0;
  println!("{} {}:{:?}",message, rule_key, rules);
  let is_match = rules.iter().any(|rule| {
    steps = 0;
    if rule.starts_with("\"") {
      steps += 1;
      return message.starts_with(&rule.trim_matches('\"'));
    } else {
      return rule.split_whitespace().all(|c| {
        let (is_match, skip) = match_rule(&message[steps..], c.to_string(), rule_map);
        steps += skip;
        return is_match;
      });
    }
  });
  return (is_match, steps);
}

fn part_1(input: &String) -> u64 {

  let mut rules: RuleMap = input.split("\n\n").nth(0).unwrap().lines().fold(HashMap::new(),|mut acc, l| {
    let splits: Vec<&str> = l.split(":").collect();
    acc.insert(splits[0].to_string(), splits[1].split("|").map(|s| { s.trim().to_string() }).collect::<Vec<String>>());
    return acc
  });
  // add end string
  let messages: Vec<String> = input.split("\n\n").nth(1).unwrap().lines().map(|l| { [l,"$"].concat() }).collect();

  // add end char $ to rules
  rules.insert("$".to_string(),vec![str!("\"$\"")]);
  *rules.get_mut(&str!("0")).unwrap() = rules.get(&str!("0")).unwrap().iter().map(|x| { format!("{} $",x) }).collect();

  let result = messages.iter().filter(|m| {
    let (is_match,_) = match_rule(m, "0".to_string(), &rules);
    return is_match;
  }).count();

  return result as u64;
}

fn part_2(input: &String) -> u64 {
  let mut rules: RuleMap = input.split("\n\n").nth(0).unwrap().lines().fold(HashMap::new(),|mut acc, l| {
    let splits: Vec<&str> = l.split(":").collect();
    acc.insert(splits[0].to_string(), splits[1].split("|").map(|s| { s.trim().to_string() }).collect::<Vec<String>>());
    return acc
  });
  // add end string
  let messages: Vec<String> = input.split("\n\n").nth(1).unwrap().lines().map(|l| { [l,"$"].concat() }).collect();

  // add end char $ to rules
  rules.insert(str!("$"),vec![str!("\"$\"")]);
  *rules.get_mut(&str!("0")).unwrap() = rules.get(&str!("0")).unwrap().iter().map(|x| { format!("{} $",x) }).collect();

  //change rule 8 and 42
  *rules.get_mut(&str!("8")).unwrap() = vec![str!("42"), str!("42 8")];
  *rules.get_mut(&str!("11")).unwrap() = vec![str!("42 31"), str!("42 11 31")];

  let result = messages.iter().filter(|m| {
    let (is_match,_) = match_rule(m, "0".to_string(), &rules);
    println!("{}",is_match);
    return is_match;
  }).count();

  return result as u64;
}

fn main() {
  let args = App::new("Advent of Code - Day 19")
  .arg(Arg::with_name("part")
  .takes_value(true)
  .required(true))
  .get_matches();
  
  let data = fs::read_to_string(FILENAME).expect("Input Error");

  let part = args.value_of("part").unwrap_or("");
  let result = match part {
    "1" => { part_1(&data.trim_end().to_string()) },
    "2" => { part_2(&data.trim_end().to_string()) },
    _ => { panic!("Select either part 1 or 2") }
  };
  println!("Result is: {}", result);
}

// run with cargo test -- --nocapture to show print output
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let data = fs::read_to_string("test-input.txt").expect("Input Error");
    let result = part_1(&data.trim_end().to_string());
    assert_eq!(result, 2);
  }

  #[test]
  fn test2_1() {
    let data = fs::read_to_string("test-input3.txt").expect("Input Error");
    // let result = part_1(&data.trim_end().to_string());
    // assert_eq!(result, 3);
    let result = part_2(&data.trim_end().to_string());
    assert_eq!(result, 1);
  }

  #[test]
  fn test2_2() {
    let data = fs::read_to_string("test-input2.txt").expect("Input Error");
    let result = part_1(&data.trim_end().to_string());
    assert_eq!(result, 3);
    let result = part_2(&data.trim_end().to_string());
    assert_eq!(result, 12);
  }
}