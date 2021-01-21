use std::fs;
use clap::{Arg, App};

const FILENAME : &str = "input.txt";

fn calc_group_count_1(data : &str) -> i32 {
  let persons : Vec<&str> = data.split("\n").collect();
  
  let mut count = 0;
  let mut questions : Vec<char> = Vec::new();
  for person in persons {
    for c in person.chars() {
      let exists = (&questions).into_iter().find(|q| { **q == c });
      match exists {
        None => {
          count += 1;
          questions.push(c);
        },
        _ => {}
      }
    }
  }
  return count
}

fn calc_group_count_2(data : &str) -> i32 {
  let persons : Vec<&str> = data.split("\n").collect();
  
  let mut questions : Vec<(char, i32)> = Vec::new();
  for person in &persons {
    for c in (*person).chars() {
      let result = (&mut questions).into_iter().find(|q| { (**q).0 == c });
      match result {
        Some(q) => {
          (*q).1 += 1;
        },
        None => {
          questions.push((c, 1));
        }
      }
    }
  }

  let group_count = questions.into_iter().fold(0, |acc, q| {
    acc + (if q.1 == persons.len() as i32 {1} else {0})
  });
  return group_count
}

fn part_1() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let groups = data.split("\n\n");

  let counts = groups.map(|g| { calc_group_count_1(g) });

  let sum = counts.fold(0, |acc, c| { acc + c });

  println!("sum of counts: {}", sum);
}

fn part_2() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let groups = data.split("\n\n");

  let counts = groups.map(|g| { calc_group_count_2(g) });

  let sum = counts.fold(0, |acc, c| { acc + c });

  println!("sum of counts: {}", sum);
}

fn main() {
  let args = App::new("Advent of Code - Day 6")
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