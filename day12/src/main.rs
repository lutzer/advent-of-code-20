use std::fs;
use clap::{Arg, App};
use std::f64::consts;

const FILENAME : &str = "input.txt";

fn read_input() -> String {
  let data = fs::read_to_string(FILENAME).expect("Input Error");
  return String::from(data.trim_end());
}

fn part_1() {
  let data = read_input();

  let instructions = data.lines().map(|l| {
    let dir = l.chars().nth(0).unwrap();
    let val : i32 = l[1..].to_string().parse().unwrap();
    return (dir, val);
  });

  let (x, y, _) = instructions.fold((0.0,0.0,0.0), |(mut x, mut y, mut heading), i| {
    match i.0 {
        'N' => { y += i.1 as f32 },
        'S' => { y -= i.1 as f32 },
        'E' => { x += i.1 as f32 },
        'W' => { x -= i.1 as f32 },
        'L' => { heading = (heading - i.1 as f32 + 360.0) % 360.0},
        'R' => { heading = (heading + i.1 as f32) % 360.0 },
        'F' => {
            x += (heading / 360.0 * consts::PI as f32 * 2.0).cos() * i.1 as f32;
            y -= (heading / 360.0 * consts::PI as f32 * 2.0).sin() * i.1 as f32;
        }
        _ => { eprintln!("Received unknown direction") }
    }
    return (x,y,heading)
  });

  println!("The manhatten distance to (0,0) is: {}", x.abs() + y.abs());
}

fn part_2() {
  let data = read_input();

  let instructions = data.lines().map(|l| {
    let dir = l.chars().nth(0).unwrap();
    let val : i32 = l[1..].to_string().parse().unwrap();
    return (dir, val);
  });

  let (sx, sy, _, _) = instructions.fold((0.0, 0.0, 10.0, 1.0), 
    |(mut sx, mut sy, mut wx, mut wy), i| {
        match i.0 {
            'N' => { wy += i.1 as f32 },
            'S' => { wy -= i.1 as f32 },
            'E' => { wx += i.1 as f32 },
            'W' => { wx -= i.1 as f32 },
            'L' => { 
                let angle = i.1 as f32 / 360.0 * consts::PI as f32 * 2.0; 
                let new_wx = angle.cos() * (wx) - angle.sin() * (wy);
                let new_wy = angle.sin() * (wx) + angle.cos() * (wy);
                wx = new_wx.round(); wy = new_wy.round()
            },
            'R' => { 
                let angle = -i.1 as f32 / 360.0 * consts::PI as f32 * 2.0;
                let new_wx = angle.cos() * (wx) - angle.sin() * (wy);
                let new_wy = angle.sin() * (wx) + angle.cos() * (wy);
                wx = new_wx.round(); wy = new_wy.round()
            },
            'F' => {
                sx += wx * i.1 as f32;
                sy += wy * i.1 as f32;
            }
            _ => { eprintln!("Received unknown direction") }
        }
        return (sx,sy,wx,wy)
    });

    println!("The manhatten distance to (0,0) is: {}", sx.abs() + sy.abs());
}

fn main() {
  let args = App::new("Advent of Code - Day 12")
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