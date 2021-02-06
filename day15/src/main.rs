use std::collections::HashMap;
use clap::{Arg, App};

fn part_1(input: &Vec<u64>) -> u64 {
  let mut spoken: HashMap<u64,u64> = HashMap::new();
  let mut turn = 1;

  for &n in input[0..input.len()-1].iter() { spoken.insert(n, turn); turn+=1; }
  let mut last_spoken = *input.last().unwrap();
  
  for _ in turn..2020 {
    match spoken.insert(last_spoken, turn) {
      Some(x) => {
        last_spoken = turn - x;
      },
      None => {
        last_spoken = 0;
      }
    }
    turn += 1;
  }

  return last_spoken;
}

fn part_2(input: &Vec<u64>) -> u64 {
  let mut spoken: HashMap<u64,u64> = HashMap::new();
  let mut turn = 1;

  for &n in input[0..input.len()-1].iter() { spoken.insert(n, turn); turn+=1; }
  let mut last_spoken = *input.last().unwrap();
  
  for _ in turn..30000000 {
    match spoken.insert(last_spoken, turn) {
      Some(x) => {
        last_spoken = turn - x;
      },
      None => {
        last_spoken = 0;
      }
    }
    turn += 1;
  }

  return last_spoken;
}

fn main() {
  let args = App::new("Advent of Code - Day 15")
  .arg(Arg::with_name("part")
  .takes_value(true)
  .required(true))
  .get_matches();
  
  let data = vec![18,8,0,5,4,1,20];

  let part = args.value_of("part").unwrap_or("");
  let result = match part {
    "1" => { part_1(&data) },
    "2" => { part_2(&data) },
    _ => { panic!("Select either part 1 or 2") }
  };
  println!("Result is: {}", result);
}