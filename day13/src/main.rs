use std::fs;
use std::cmp;
use std::io::{self, Write};
use clap::{Arg, App};

const FILENAME: &str = "input.txt";

fn read_input() -> String {
  let data = fs::read_to_string(FILENAME).expect("Input Error");
  return String::from(data.trim_end());
}

fn calculate_smallest_common_produkt(factors: &Vec<u128>) -> u128 {
  return factors.iter().fold(1, |acc, &v| {
    let min = cmp::min(acc, v);
    let max = cmp::max(acc, v);
    let mut product : u128 = min;
    while product % max != 0 {
      product += min;
    }
    return product;
  });
}

fn part_1() {
  let data = read_input();

  let departure_time: i32 = data.lines().nth(0).unwrap().parse().unwrap();
  let buses: Vec<i32> = data.lines().nth(1).unwrap()
    .split(",")
    .filter(|&s| { s != "x" })
    .map(|s| { s.parse::<i32>().unwrap() })
    .collect();

  let departure_remainders = buses.iter().map(|bus| {
    return (bus - (departure_time % bus)) % bus;
  });

  let (bus, waiting_time): (i32,i32) = departure_remainders.enumerate()
    .fold((-1,std::i32::MAX), |acc, (i, time)| {
      if acc.1 > time { (buses[i], time) } else { acc }
    });

  println!("Bus {} leaves after {} minutes: {}", bus, waiting_time, bus * waiting_time);
}

fn part_2() {
  let data = read_input();

  let buses: Vec<u32> = data.lines().nth(1).unwrap()
    .split(",")
    .map(|s| { s.parse::<u32>().unwrap_or(1) })
    .collect();

  let mut departure_time: u128 = 1;
  let mut increment: u128 = 1;
  let mut previous_buses_on_time = vec![false; buses.len()];
  loop {
    let buses_on_time: Vec<(u32,bool)> = buses.iter().enumerate().map(|(i,&bus)| {
      let rest = (departure_time + i as u128) % bus as u128;
      return (bus, rest == 0)
    }).collect();

    if !buses_on_time.iter().any(|x| { x.1 != true }) {
      break;
    }

    let mut changed_buses: Vec<u128> = buses_on_time.iter()
      .zip(&previous_buses_on_time)
      .filter(|(current, &prev)| { current.1 != prev })
      .map(|x| { x.0.0 as u128 })
      .collect();

    if changed_buses.len() > 0 {
      changed_buses.push(increment);
      increment = calculate_smallest_common_produkt(&changed_buses);
    }

    previous_buses_on_time = buses_on_time.iter().map(|x| { x.1 }).collect();

    departure_time += increment;
    print!(".");
    io::stdout().flush().unwrap();
  }

  println!("\ndeparture time: {}", departure_time);
}

fn main() {
  let args = App::new("Advent of Code - Day 13")
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