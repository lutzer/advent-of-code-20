use std::fs;
use std::cmp;
use clap::{Arg, App};
use bit_vec::BitVec;

const FILENAME : &str = "input.txt";

struct Ticket {
  row: i32,
  column: i32,
  id: i32
}

fn parse_ticket_data(line: &str) -> Ticket {
  let mut row = (0,127);
  for i in 0..7 {
    let c = line.chars().nth(i).unwrap();
    if c == 'F' {
      row.1 = row.0 + (row.1-row.0) / 2;
    } else if c == 'B' {
      row.0 = row.0 + (row.1-row.0) / 2 + 1;
    }
  }
  let mut col = (0,7);
  for i in 7..10 {
    let c = line.chars().nth(i).unwrap();
    if c == 'L' {
      col.1 = col.0 + (col.1-col.0) / 2;
    } else if c == 'R' {
      col.0 = col.0 + (col.1-col.0) / 2 + 1;
    }
  }
  return Ticket{ row: row.0, column: col.0, id: col.0 + row.0 * 8};
}

fn part_1() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let lines : Vec<&str> = data.split("\n").collect();

  let tickets : Vec<Ticket> = lines.iter().map(|l| { parse_ticket_data(l) }).collect();

  let max_id = tickets.into_iter().fold(0, |acc, t| {
    cmp::max(acc,t.id)
  });

  println!("maximum id: {}", max_id);
}

fn part_2() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let lines : Vec<&str> = data.split("\n").collect();

  let tickets : Vec<Ticket> = lines.iter().map(|l| { parse_ticket_data(l) }).collect();

  // create bit array with all seats
  let mut seats_taken = BitVec::from_elem(128 * 8, false);

  // set taken seats
  for ticket in tickets {
    seats_taken.set(ticket.id as usize, true);
  }

  // search your seat
  for i in 1..(seats_taken.len()-1) {
    if !seats_taken[i] && seats_taken[i-1] && seats_taken[i+1] {
      println!("your seat is: {}", i);
      break;
    }
  }
}

fn main() {
  let args = App::new("Advent of Code - Day 4")
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