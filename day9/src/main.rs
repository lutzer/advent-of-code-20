use std::fs;
use clap::{Arg, App};

const FILENAME : &str = "input.txt";
const PREAMBLE_SIZE : usize = 25;

fn validate_number(number_list : &Vec<i64>, index : usize, preamble_size : usize) -> (i64,bool) {
  for i in index-preamble_size..index {
    for j in i+1..index {
      if number_list[i] + number_list[j] == number_list[index] {
        return (number_list[index],true);
      }
    }
  }
  return (number_list[index],false);
}

fn find_contigous_number_range(number_list : &Vec<i64>, number: i64) -> Option<(usize,usize)> {
  for i in 0..number_list.len() {
    let mut sum = number_list[i];
    for j in i+1..number_list.len() {
      sum += number_list[j];
      if sum == number {
        return Some((i, j));
      } else if sum > number {
        break;
      }
    }
  }
  return None;
}

fn part_1() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let numbers : Vec<i64> = data.trim_end().split("\n").map(|line| { line.parse().expect("Error parsing number") }).collect();

  let mut valid_numbers = (PREAMBLE_SIZE..numbers.len()).map(|x| { 
    validate_number(&numbers, x, PREAMBLE_SIZE)
  });

  let first_valid_number = valid_numbers.find(|v| { v.1 == false });

  println!("first invalid number is: {}", first_valid_number.unwrap().0)
}

fn part_2() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let numbers : Vec<i64> = data.trim_end().split("\n").map(|line| { line.parse().expect("Error parsing number") }).collect();

  let mut valid_numbers = (PREAMBLE_SIZE..numbers.len()).map(|x| { 
    validate_number(&numbers, x, PREAMBLE_SIZE)
  });

  let first_valid_number = valid_numbers.find(|v| { v.1 == false }).unwrap().0;

  let contigous_range = find_contigous_number_range(&numbers, first_valid_number).unwrap();

  let range_numbers = &numbers[contigous_range.0..contigous_range.1];
  let max = range_numbers.into_iter().max().unwrap();
  let min = range_numbers.into_iter().min().unwrap();

  println!("Number range is: {:?}, sum is: {}", range_numbers, max + min);

}

fn main() {
  let args = App::new("Advent of Code - Day 9")
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