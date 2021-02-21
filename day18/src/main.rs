use std::fs;
use clap::{Arg, App};

const FILENAME: &str = "input.txt";

enum Operator {
  Add, Multiply
}

fn calculate_instructions_1(input: &str, mut i: usize) -> (i64, usize) {

  let splits: Vec<&str> = input.split(" ").collect();

  let mut operator = Operator::Add;
  let mut result = 0;

  while i < splits.len() {
    match splits[i] {
      "+" => { operator = Operator::Add; },
      "*" => { operator = Operator::Multiply },
      "(" => {
        let (r, skip_to) = calculate_instructions_1(input, i+1);
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

fn delimited_paranthesis(input: &[&str]) -> (usize, usize) {
  let mut start = 0;
  let mut opened = 0;
  for (i,&c) in input.iter().enumerate() {
    match c {
      "(" => { opened +=1; start = i;  }
      ")" => { opened -=1; if opened == 0 { return (start, i); } },
      _ => { }
    }
  }
  return (0,0);
}

fn calculate_instructions_2(input: &str) -> i64 {
  let splits: Vec<&str> = input.split(" ").collect();

  // let brackets = delimited_paranthesis(input);

  let mut result: Vec<String> = vec![];
  let mut i = 0;
  while i < splits.len() {
    match splits[i] {
      "(" => {
        let brackets = delimited_paranthesis(&splits[i..]);
        result.push(splits[brackets.0+i+1..brackets.1+i].join(" "));
        i += brackets.1;
      },
      _ => {
        result.push(splits[i].to_string());
       }
    }
    i += 1;
  }

  println!("{:?}:{:?}", splits, result);

  // let mut operator = Operator::Add;
  // let mut result = 0;
  
  // let mut mult_mem: Option<i64> = None;

  // let mut i = 0;
  // for (i,s) in splits.iter().enumerate() {
  //   match s {
  //     "+" => { operator = Operator::Add; },
  //     "*" => { operator = Operator::Multiply },
  //     _ => { }
  //     } 
  //   }
  //   println!("{}:{:?} {}:{}", result, mult_mem, i, splits[i]);
  //   i += 1;
  // }
  // let (s,e) = delimited_paranthesis(input);
  // println!("{}:{}:{}", s, e, input);


  // assert_eq!(parser("abc|efg"), Ok(("", "|")));
  return 0;
}

fn part_1(input: &String) -> i64 {
  let results: Vec<i64> = input.lines().map(|l| { 
    let input_edited = l.replace("(", "( ").replace(")"," )");
    let (result, _) = calculate_instructions_1(&input_edited, 0);
    return result;
  }).collect();
  return results.iter().sum();
}

fn part_2(input: &String) -> i64 {
  let results: Vec<i64> = input.lines().map(|l| { 
    let input_edited = l.replace("(", "( ").replace(")"," )");
    let result = calculate_instructions_2(&input_edited);
    return result;
  }).collect();
  return results.iter().sum();
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
    // assert_eq!(part_1(&"2 * 3 + 4".to_string()), 10);
    // assert_eq!(part_1(&"2 * 3 + (4 * 5)".to_string()), 26);
    // assert_eq!(part_1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 437);
    // assert_eq!(part_1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 12240);
    // assert_eq!(part_1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 13632);
  }

  #[test]
  fn test_2() {
    assert_eq!(part_2(&"2 * 3 + 4".to_string()), 14);
    //assert_eq!(part_2(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()), 51);
    // assert_eq!(part_2(&"2 * 3 + (4 * 5)".to_string()), 46);
    // assert_eq!(part_2(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 1445);
    // assert_eq!(part_2(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 669060);
    // assert_eq!(part_2(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 23340);
  }
}