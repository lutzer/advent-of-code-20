use std::fs;
use clap::{Arg, App};

const FILENAME : &str = "input.txt";
const INFINITE_LOOP_MAX_STEPS : i32 = 1000;

macro_rules! str {
  ($e:expr) => { String::from($e) };
}

#[derive(Debug, Clone)]
struct Instruction {
  cmd: String, 
  arg: i32
}

fn wait_for_key() {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap(); 
}

fn parse_instruction(line : &str) -> Instruction {
  let parts : Vec<&str> = line.split(" ").collect();
  return Instruction{ cmd: str!(parts[0]), arg: parts[1].parse().expect("Error parsing instruction.")};
}

fn run_instructions(instructions : &Vec<Instruction>) -> i32 {
  let mut acc = 0;
  let mut i : i32 = 0;
  let mut instruction_executed = vec![false; instructions.len()];
  
  loop {
    // break if instructions is executed a second time
    if instruction_executed[i as usize] {
      break;
    }
    instruction_executed[i as usize] = true;

    let current = &instructions[i as usize];
    let cmd = &current.cmd[..];
    match cmd {
      "acc" => {
        acc += current.arg;
        i += 1;
      },
      "jmp" => {
        i += current.arg;
      },
      "nop" => {
        i += 1;
      }
      _ => { panic!("Encountered unknown instruction."); }
    }
  }
  return acc
}

fn get_possible_permutations_indices(instructions : &Vec<Instruction>) -> Vec<usize> {
  let mut permutations : Vec<usize> = vec![];
  for i in 0..instructions.len() {
    if instructions[i].cmd == "nop" || instructions[i].cmd == "jmp" {
      permutations.push(i);
    }
  }
  return permutations;
}

fn run_changed_instructions(instructions : &Vec<Instruction>, changed_index : usize) -> Option<i32> {
  let mut acc = 0;
  let mut i : i32 = 0;
  let mut step : i32 = 0;
  while (i as usize) < instructions.len() {

    let current = &instructions[i as usize];
    let mut cmd = &current.cmd[..];
    
    if changed_index == i as usize && cmd == "jmp" {
      cmd = "nop";
    } else if changed_index == i as usize && cmd == "nop" {
      cmd = "jmp"
    }

    match cmd {
      "acc" => {
        acc += current.arg;
        i += 1;
      },
      "jmp" => {
        i += current.arg;
      },
      "nop" => {
        i += 1;
      }
      _ => { panic!("Encountered unknown instruction."); }
    }

    step += 1;
    if step > INFINITE_LOOP_MAX_STEPS { 
      return None
    }
  }
  return Some(acc)
}

fn part_1() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();
  
  let instructions : Vec<Instruction> = data
    .trim_end()
    .split("\n").map(|l| { parse_instruction(l)}).collect();

  let acc = run_instructions(&instructions);
  
  println!("program result: {}", acc);
}

fn part_2() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let instructions : Vec<Instruction> = data
    .trim_end()
    .split("\n").map(|l| { parse_instruction(l)}).collect();

  let permutation_indices = get_possible_permutations_indices(&instructions);

  for i in permutation_indices {
    let result = run_changed_instructions(&instructions, i);
    match result {
      None => {},
      Some(x) => {
        println!("program result: {}", x);
        return;
      }
    }
  }
  eprintln!("did not get any valid program result.");
}

fn main() {
  let args = App::new("Advent of Code - Day 8")
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