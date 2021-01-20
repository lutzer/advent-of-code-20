use std::fs;
use std::string;
use std::option;
use clap::{Arg, App};

const ROW_LENGTH : usize = 31;
const FILENAME : &str = "input.txt";

fn get_char_at(data : &string::String, pos : (usize,usize)) -> option::Option<char> {
  return data.chars().nth(pos.0 + pos.1 * ROW_LENGTH)
}

fn part_1() {
  println!("Reading data from file {}.", FILENAME);
  let data = fs::read_to_string(FILENAME)
    .unwrap()
    .replace('\n', "");
  
  let mut position = (0, 0);
  let mut count = 0;

  loop {
    match get_char_at(&data, position) {
      Some(c) => { 
        // println!("({},{}) {}", position.0, position.1, c);
        count = if c == '#' {count + 1} else { count };
        position.0 = (position.0 + 3) % ROW_LENGTH;
        position.1 += 1
      },
      None => { 
        println!("reached end. found {} trees on the way.", count);
        break;
      }
    }
  }
}

fn part_2() {
  println!("part2");
}

fn main() {
  let args = App::new("Advent of Code - Day 3")
    .arg(Arg::with_name("day")
      .takes_value(true))
    .get_matches();

  let day = args.value_of("day").unwrap_or("");

  match day {
    "1" => part_1(),
    "2" => part_2(),
    _ => panic!("Select either part 1 or 2")
  }
}