use std::fs;
use std::collections::HashMap;
use clap::{Arg, App};
use regex::Regex;

const FILENAME: &str = "input.txt";

fn parse_command(str: &str) -> (usize,u64) {
  let re = Regex::new(r"^mem\[([0-9]+)\]\s=\s([0-9]+)").unwrap();
  let capture_groups = re.captures(str).unwrap();
  let address: usize = capture_groups.get(1).unwrap().as_str().parse().unwrap();
  let val: u64 = capture_groups.get(2).unwrap().as_str().parse().unwrap();
  return (address,val);
}

fn get_floating_addresses(address: u64, mask: &str, mask_pos: usize) -> Vec<u64> {
  let mut addresses = vec![];
  match mask.chars().rev().nth(mask_pos) {
    Some('X') => {
      let address_0 = address & !(1 << mask_pos);
      let address_1 = address | (1 << mask_pos);
      if address_0 != address { addresses.push(address_0); }
      if address_1 != address { addresses.push(address_1); }
      addresses.extend(get_floating_addresses(address_1, mask, mask_pos + 1));
      addresses.extend(get_floating_addresses(address_0, mask, mask_pos + 1));
    },
    None => {
     }
    _ => {
      addresses.extend(get_floating_addresses(address, mask, mask_pos + 1));
    }
  };
  return addresses;
}

fn part_1(input: &String) -> u64 {

  let mut memory: HashMap<usize,u64> = HashMap::new();
  let mut mask = "";
  for line in input.lines() {
    let parts: Vec<&str> = line.split(" = ").collect();
    match parts[0] {
      "mask" => { 
        mask = parts[1];
      },
      _ => {
        let cmd = parse_command(&line);
        let val = mask.chars().rev().enumerate().fold(cmd.1, |acc,(i,c)| {
          match c {
            '0' => { acc & !(1 << i) },
            '1' => { acc | 1 << i },
            _ => { acc }
          }
        });
        memory.insert(cmd.0, val);
      }
    }
  }

  let sum = memory.into_iter().fold(0, |acc,x| { acc + x.1 } );
  println!("Sum of all memory entries is: {}", sum);
  return sum;
}

fn part_2(input: &String) -> u64 {

  let mut memory: HashMap<usize,u64> = HashMap::new();
  let mut mask = "";
  for line in input.lines() {
    let parts: Vec<&str> = line.split(" = ").collect();
    match parts[0] {
      "mask" => { 
        mask = parts[1];
      },
      _ => {
        let cmd = parse_command(&line);
        let address = mask.chars().rev().enumerate().fold(cmd.0 as u64, |acc,(i,c)| {
          match c {
            '1' => { acc | 1 << i },
            _ => { acc }
          }
        });
        let addresses = [vec![address], get_floating_addresses(address, &mask, 0)].concat();
        for a in addresses {
          memory.insert(a as usize, cmd.1);
        }
      }
    }
  }

  let sum = memory.into_iter().fold(0, |acc,x| { acc + x.1 } );
  println!("Sum of all memory entries is: {}", sum);
  return sum;
}

fn main() {
  let args = App::new("Advent of Code - Day ?")
  .arg(Arg::with_name("part")
  .takes_value(true)
  .required(true))
  .get_matches();
  
  let part = args.value_of("part").unwrap_or("");

  let mut data = fs::read_to_string(FILENAME).expect("Input Error");
  data = String::from(data.trim_end());
  
  match part {
    "1" => { part_1(&data); },
    "2" => { part_2(&data); },
    _ => eprintln!("Select either part 1 or 2")
  }
}

// run with cargo test --nocapture to show print output
#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_part_1() {
    let input = String::from(indoc! {"
      mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
      mem[8] = 11
      mem[7] = 101
      mem[8] = 0
    "});
    let result = part_1(&input);
    assert_eq!(result, 165);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc! {"
      mask = 000000000000000000000000000000X1001X
      mem[42] = 100
      mask = 00000000000000000000000000000000X0XX
      mem[26] = 1
    "});
    let result = part_2(&input);
    assert_eq!(result, 208);
  }
}