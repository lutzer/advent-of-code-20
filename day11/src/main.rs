use std::fs;
use clap::{Arg, App};

macro_rules! neighbours {
  ($i:ident,$j:ident) => { 
    [($i-1, $j-1), ($i-1, $j), ($i-1, $j+1), ($i, $j-1), ($i, $j+1), ($i+1,$j-1), ($i+1, $j), ($i+1, $j+1)]
  };
}

const FILENAME : &str = "input.txt";

fn read_input() -> String {
  let data = fs::read_to_string(FILENAME).expect("Input Error");
  return String::from(data.trim_end());
}

type SeatMap = Vec<char>;

fn take_seats(smap: &SeatMap, width: usize, height: usize) -> SeatMap {
  let mut new_map : SeatMap = smap.clone();
  for i in 1..width-1 {
    for j in 1..height-1 {
      match smap[i+j*width] {
        'L' => { 
          let occupied_neighbours = neighbours!(i,j).iter().fold(0, |acc,&t| {
            if smap[t.0+t.1*width] == '#' {acc + 1} else {acc}
          });
          new_map[i+j*width] = if occupied_neighbours == 0 {'#'} else {'L'}
        },
        '#' => {
          let occupied_neighbours = neighbours!(i,j).iter().fold(0, |acc,&t| {
            if smap[t.0+t.1*width]== '#' {acc + 1} else {acc}
          });
          new_map[i+j*width] = if occupied_neighbours >= 4 {'L'} else {'#'}
        }
        _ => {}
      }
    }
  }
  return new_map
}

fn take_seats_by_sight(smap: &SeatMap, width: usize, height: usize) -> SeatMap {

  fn check_line_of_sight(smap: &SeatMap, width: usize, (mut i,mut j): (usize, usize), dir: usize) -> bool {
    loop {
      let new_ij = neighbours!(i,j)[dir]; i = new_ij.0; j = new_ij.1;
      match smap[i+j*width] {
        '#' => { return true }
        'L'|'x' => { return false }
        _ => {}
      }
    }
  }

  let mut new_map : SeatMap = smap.clone();
  for i in 1..width-1 {
    for j in 1..height-1 {
      match smap[i+j*width] {
        'L' => { 
          let occupied_neighbours = (0..8).fold(0, |acc, dir| {
            if check_line_of_sight(smap, width, (i,j), dir) {acc + 1} else {acc}
          });
          new_map[i+j*width] = if occupied_neighbours == 0 {'#'} else {'L'}
        },
        '#' => {
          let occupied_neighbours = (0..8).fold(0, |acc, dir| {
            if check_line_of_sight(smap, width, (i,j), dir) {acc + 1} else {acc}
          });
          new_map[i+j*width] = if occupied_neighbours >= 5 {'L'} else {'#'}
        }
        _ => {}
      }
    }
  }
  return new_map
}

fn map_is_equal(map1 : &SeatMap, map2 : &SeatMap) -> bool {
  for i in 1..map1.len()-1 {
    if map1[i] != map2[i] {
      return false;
    }
  }
  return true;
}

fn print_map(map : &SeatMap, width: usize) {
  for i in 0..map.len() {
    if i % width == 0 { println!("") };
    print!("{}",map[i]);
  }
  println!("")
}

fn part_1() {
  let data = read_input();

  let width = data.lines().next().unwrap().len();
  let height = data.lines().count();

  let mut seat_map : Vec<char> = ["x".repeat(width+1), data, "x".repeat(width+1)]
    .join("\n")
    .replace("\n","xx")
    .chars()
    .collect();

  loop {
    let new_seats = take_seats(&seat_map, width+2, height+2);
    if map_is_equal(&new_seats, &seat_map) {
      break;
    }
    seat_map = new_seats;
  }

  let occupied_seats = seat_map.iter().filter(|&c| { *c == '#' }).count();

  println!("occupied seats by neighbours: {}", occupied_seats);
}

fn part_2() {
  let data = read_input();

  let width = data.lines().next().unwrap().len();
  let height = data.lines().count();

  let mut seat_map : Vec<char> = ["x".repeat(width+1), data, "x".repeat(width+1)]
    .join("\n")
    .replace("\n","xx")
    .chars()
    .collect();

  loop {
    let new_seats = take_seats_by_sight(&seat_map, width+2, height+2);
    if map_is_equal(&new_seats, &seat_map) {
      break;
    }
    seat_map = new_seats;
  }

  let occupied_seats = seat_map.iter().filter(|&c| { *c == '#' }).count();

  println!("occupied seats by neighbours sight: {}", occupied_seats);

}

fn main() {
  let args = App::new("Advent of Code - Day 11")
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