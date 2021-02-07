use std::fs;
use clap::{Arg, App};
use std::collections::HashMap;

const FILENAME: &str = "input.txt";

#[derive(Debug, Copy, Clone)]
struct Cube {
  position: [i32; 3],
  active: bool
}

impl Cube {
  fn is_in_position(&self, position: &[i32]) -> bool {
    return position.iter().enumerate().all(|(i,p)| {
      return *p == self.position[i];
    })
  }

  fn get_hash(&self) -> String {
    return format!("{}:{}:{}", self.position[0], self.position[1], self.position[2]);
  }
}

struct Cube4D {
  position: [i32; 4],
  active: bool
}

fn print_map(cubes: &Vec<Cube>, width: u32, height: u32, start: (i32, i32, i32)) {
  for y in 0..height {
    println!();
    for x in 0..width {
      if cubes.iter().any(|&c| { c.is_in_position(&[start.0 + x as i32, start.1 + y as i32, start.2]) }) {
        print!("#");
      } else {
        print!(".");
      }
    }
  }
  println!();
}

fn get_neighbours<'a>(center: &Cube, cubes: &Vec<Cube>) -> Vec<Cube> {
  return (0..27).filter(|&i| { i != 13 }).map(|i| {
    let x = i % 3 - 1 + center.position[0];
    let y = (i / 3) % 3 - 1 + center.position[1];
    let z = i / (3*3) - 1 + center.position[2];
    return Cube {
      position: [x, y, z],
      active: cubes.iter().any(|c| { c.is_in_position(&[x,y,z]) })
    };
  }).collect();
}

fn part_1(input: &String) -> u64 {
  // load active cubes from input
  let (mut active_cubes,_,_) = input.chars().fold((vec![],0,0), |(mut cubes,i,j), c| {
    if c == '\n' {
      return (cubes, 0, j+1);
    }
    if c == '#' {
      cubes.push(Cube{
        position: [i, j, 0],
        active: true
      });
    }
    return (cubes, i+1, j);
  });

  for _ in 0..6 {
    // insert all neighbours of active cubes and count number of active neighbours
    let check_cubes: HashMap<String,(u32, Cube)> = active_cubes.iter().fold(HashMap::new(),|mut map, cube| {
      map.entry(cube.get_hash()).or_insert((0, *cube));
      let neighbours = get_neighbours(cube, &active_cubes);
      for n in neighbours {
        let hash = n.get_hash();
        map.entry(hash).or_insert((0, n)).0 += 1;
      }
      return map;
    });

    // set active cubes
    active_cubes = check_cubes.iter().fold(vec![],|mut acc, (_,(count, cube))| {
      if cube.active && *count == 2 || *count == 3 {
        acc.push(Cube{ position: cube.position, active: true});
      } else if !cube.active && *count == 3 {
        acc.push(Cube{ position: cube.position, active: true });
      } 
      return acc;
    });
  }  

  return active_cubes.len() as u64;
}

fn part_2(input: &String) -> u64 {
  //load active cubes from input
  let (mut active_cubes,_,_) = input.chars().fold((vec![],0,0), |(mut cubes,i,j), c| {
    if c == '\n' {
      return (cubes, 0, j+1);
    }
    if c == '#' {
      cubes.push(Cube4D{
        position: [i, j, 0, 0],
        active: true
      });
    }
    return (cubes, i+1, j);
  });

  return 0
}

fn main() {
  let args = App::new("Advent of Code - Day 17")
  .arg(Arg::with_name("part")
  .takes_value(true)
  .required(true))
  .get_matches();
  
  let data = fs::read_to_string(FILENAME).expect("Input Error");

  let part = args.value_of("part").unwrap_or("");
  let result = match part {
    "1" => { part_1(&data.trim_end().to_string()) },
    "2" => { part_2(&data.trim_end().to_string()) },
    _ => { panic!("Select either part 1 or 2") }
  };
  println!("Result is: {}", result);
}

// run with cargo test -- --nocapture to show print output
#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_part_1() {
    let input = String::from(indoc! {"
      .#.
      ..#
      ###
    "});
    let result = part_1(&input);
    assert_eq!(result, 112);
  }

  #[test]
  fn test_part_2() {
    let input = String::from(indoc! {"
      .#.
      ..#
      ###
    "});
    let result = part_2(&input);
    assert_eq!(result, 848);
  }
}