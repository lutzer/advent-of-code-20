use std::fs;
use clap::{Arg, App};

const FILENAME : &str = "input.txt";

fn read_input() -> String {
  let data = fs::read_to_string(FILENAME).expect("Input Error");
  return String::from(data.trim_end());
}

fn calculate_adapter_combinations(adapters : &Vec<u32>) -> usize {

  // calculate how many possible adapters fit within the next 4 adapters in the chain
  let adapters_in_range : Vec<usize> = [adapters.clone(), vec![std::u32::MAX;3]].concat().windows(4).map(|a| {
    a[1..].iter().filter(|&x| {(*x - a[0]) <= 3 }).count()
  }).collect();

  // calculate possible combinations for any point in the vector from the back
  let combinations : Vec<usize> = adapters_in_range.iter().rev().fold(vec![], |acc, &a| {
    let c = acc[acc.len()-a..].iter().fold(0, |sum, v| { sum + v});
    [acc, vec![if c > 0 {c} else {1}]].concat()
  });

  return *combinations.last().unwrap();
}
      
fn part_1() {
  let data = read_input();
  let mut jolt_adapters : Vec<u32> = data.lines().map(|l| { l.parse().unwrap() }).collect();
  
  // sort adapters
  jolt_adapters.sort();
  
  // get differences
  let differences : Vec<u32> = jolt_adapters.windows(2).map(|t| { t[1] - t[0] }).collect();
  
  let diffs_1 = differences.iter().filter(|&d| { *d == 1 }).count() + 1;
  let diffs_3 = differences.iter().filter(|&d| { *d == 3 }).count() + 1;
  
  println!("Mutlipliction of number of 1({}) and 3({}) differences: {}", diffs_1, diffs_3, diffs_1 * diffs_3);
}

fn part_2() {
  let data = read_input();
  let mut jolt_adapters : Vec<u32> = data.lines().map(|l| { l.parse().unwrap() }).collect();
  
  // sort adapters
  jolt_adapters.sort();
  jolt_adapters.insert(0,0);
  
  let combinations_count = calculate_adapter_combinations(&jolt_adapters);
  
  println!("Possible combinations of adapters: {:?}", combinations_count);
}

fn main() {
  let args = App::new("Advent of Code - Day 10")
  .arg(Arg::with_name("part")
  .takes_value(true)
  .required(true))
  .get_matches();
  
  let part = args.value_of("part").unwrap_or("");
  
  match part {
    "1" => part_1(),
    "2" => part_2(),
    _ =>  eprintln!("Select either part 1 or 2")
  }
}