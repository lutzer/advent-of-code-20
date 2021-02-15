use std::fs;
use clap::{Arg, App};

const FILENAME: &str = "input.txt";

enum Operator {
  Add, Multiply
}

fn calculate_instructions(input: &str, mut i: usize) -> (i64, usize) {

  let splits: Vec<&str> = input.split(" ").collect();

  let mut operator = Operator::Add;
  let mut result = 0;

  while i < splits.len() {
    match splits[i] {
      "+" => { operator = Operator::Add; },
      "*" => { operator = Operator::Multiply },
      "(" => {
        let (r, skip_to) = calculate_instructions(input, i+1);
        match operator {
          Operator::Add => {
            result += r;
          },
          Operator::Multiply => {
            result *= r;
          }
        }
        i = skip_to;
       },
      ")" => {
        return (result, i);
       },
      _ => {
        let value = splits[i].parse::<i64>().expect("Could not parse number");
        match operator {
          Operator::Add => {
            result += value;
          },
          Operator::Multiply => {
            result *= value;
          }
        }
      }
    }
    i += 1;
  }
  return (result, i)
}

fn part_1(input: &String) -> i64 {
  let results: Vec<i64> = input.lines().map(|l| { 
    let input_edited = l.replace("(", "( ").replace(")"," )");
    let (result, _) = calculate_instructions(&input_edited, 0);
    return result;
  }).collect();
  return results.iter().sum();
}

fn part_2(input: &String) -> i64 {
  return 0
}

fn main() {
  let args = App::new("Advent of Code - Day 15")
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
  fn test_1() {
    assert_eq!(part_1(&"2 * 3 + 4".to_string()), 10);
    assert_eq!(part_1(&"2 * 3 + (4 * 5)".to_string()), 26);
    assert_eq!(part_1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 437);
    assert_eq!(part_1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 12240);
    assert_eq!(part_1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 13632);
  }

  #[test]
  fn test_2() {
    // assert_eq!(part_1(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()), 51);
    // assert_eq!(part_1(&"2 * 3 + (4 * 5)".to_string()), 46);
    // assert_eq!(part_1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 1445);
    // assert_eq!(part_1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 669060);
    // assert_eq!(part_1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 23340);
  }
}