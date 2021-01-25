use std::fs;
use clap::{Arg, App};

const FILENAME : &str = "input.txt";

fn read_input() -> String {
  let data = fs::read_to_string(FILENAME).expect("Input Error");
  return String::from(data.trim_end());
}

fn part_1() {
  let data = read_input();
}

fn part_2() {
  let data = read_input();
}

fn main() {
  let args = App::new("Advent of Code - Day ?")
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