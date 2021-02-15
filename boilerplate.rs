use std::fs;
use clap::{Arg, App};

const FILENAME: &str = "input.txt";

fn part_1(input: &String) -> u64 {
  return 0
}

fn part_2(input: &String) -> u64 {
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
  use indoc::indoc;

  #[test]
  fn test_1() {
    let input = String::from(indoc! {"
      .#.
      ..#
      ###
    "});
    let result = part_1(&input);
    assert_eq!(result, 112);
  }

  #[test]
  fn test_2() {
    let input = String::from(indoc! {"
      .#.
      ..#
      ###
    "});
    let result = part_2(&input);
    assert_eq!(result, 848);
  }
}