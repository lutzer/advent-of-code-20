use std::fs;
use std::string;
use std::option;
use clap::{Arg, App};

const ROW_LENGTH : usize = 31;
const FILENAME : &str = "input.txt";

fn get_char_at(data : &string::String, pos : (usize,usize)) -> option::Option<char> {
  return data.chars().nth(pos.0 + pos.1 * ROW_LENGTH)
}

fn traverse_tree(data : &string::String, slope : &(usize, usize)) -> i32 {
  let mut position = (0, 0);
  let mut count = 0;

  loop {
    match get_char_at(&data, position) {
      Some(c) => { 
        // println!("({},{}) {}", position.0, position.1, c);
        count = if c == '#' {count + 1} else { count };
        position.0 = (position.0 + slope.0) % ROW_LENGTH;
        position.1 += slope.1
      },
      None => { 
        break;
      }
    }
  }
  return count;
}

fn part_1() {
  println!("Reading data from file {}.", FILENAME);
  let data = fs::read_to_string(FILENAME)
    .unwrap()
    .replace('\n', "");
  
    let tree_count = traverse_tree(&data, &(3, 1));
    println!("Reached end. found {} trees on the way.", tree_count);
}

fn part_2() {
  println!("Reading data from file {}.", FILENAME);
  let data = fs::read_to_string(FILENAME)
    .unwrap()
    .replace('\n', "");

  let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];

  let tree_counts = slopes.iter().map(|slope| { traverse_tree(&data, &slope)});

  let multiplied_counts = tree_counts.fold(1 as i128, |acc, val| acc * (val as i128));

  println!("Multiplied tree counts: {}", multiplied_counts);
}

fn main() {
  let args = App::new("Advent of Code - Day 3")
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