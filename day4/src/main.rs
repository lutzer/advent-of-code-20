use std::fs;
use std::string;
use clap::{Arg, App};
use regex::Regex;

const FILENAME : &str = "input.txt";

#[derive(Debug, Default)]
struct Passport {
  byr: Option<String>,
  iyr: Option<String>,
  eyr: Option<String>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<String>
}

fn parse_hgt(str : &String) -> (i32, String) {
  if str.contains("cm") {
    let val = str.replace("cm","")
      .parse().unwrap_or(-1);
    return (val, String::from("cm"));
  }
  if str.contains("in") {
    let val = str.replace("in","")
      .parse().unwrap_or(-1);
    return (val, String::from("in"));
  }
  return (-1, String::from(""));
}

impl Passport {
  #[allow(dead_code)]
  fn print(&self) {
    println!("byr:{:?}, iry:{:?}, eyr:{:?}, hgt:{:?}, hcl:{:?}, ecl:{:?}, pid: {:?}, cid: {:?} ", 
      self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid, self.cid);
  }

  fn validate1(&self) -> bool {
    return self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() && self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some() && self.pid.is_some()
  }

  fn validate2(&self) -> bool {
    let byr_check = match &self.byr {
      Some(v) => {
        let n : i64 = v.parse().unwrap_or(-1);
        n >= 1920 && n <= 2002
      },
      None => false
    };

    let iyr_check = match &self.iyr {
      Some(v) => {
        let n : i64 = v.parse().unwrap_or(-1);
        n >= 2010 && n <= 2020
      },
      None => false
    };

    let eyr_check = match &self.eyr {
      Some(v) => {
        let n : i64 = v.parse().unwrap_or(-1);
        n >= 2020 && n <= 2030
      },
      None => false
    };

    let hgt_check = match &self.hgt {
      Some(v) => {
        let parsed_hgt = parse_hgt(&v);
        (parsed_hgt.1 == "cm" && parsed_hgt.0 >= 150 && parsed_hgt.0 <= 193) ||
        (parsed_hgt.1 == "in" && parsed_hgt.0 >= 59 && parsed_hgt.0 <= 76)
      },
      None => false
    };

    let hcl_check = match &self.hcl {
      Some(v) => {
        let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        re.is_match(v)
      },
      None => false
    };

    let ecl_check = match &self.ecl {
      Some(v) => {
        match v as &str {
          "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
          _ => false
        }
      },
      None => false
    };

    let pid_check = match &self.pid {
      Some(v) => {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        re.is_match(v)
      },
      None => false
    };
    return byr_check && iyr_check && eyr_check && hgt_check && hcl_check && ecl_check && pid_check;
  }
}

fn parse_passport_data(data : &string::String) -> Vec<Passport> {

  fn parse_entry(entry : &str) -> Passport {
    let fields : Vec<&str> = entry.split(" ").collect();
    let mut passport = Passport::default();
    for field in fields {
      let v : Vec<&str> = field.split(":").collect();
      match v[0] {
        "byr" => { passport.byr = Some(String::from(v[1])) },
        "iyr" => { passport.iyr = Some(String::from(v[1])) },
        "eyr" => { passport.eyr = Some(String::from(v[1])) },
        "hgt" => { passport.hgt = Some(String::from(v[1])) },
        "hcl" => { passport.hcl = Some(String::from(v[1])) },
        "ecl" => { passport.ecl = Some(String::from(v[1])) },
        "pid" => { passport.pid = Some(String::from(v[1])) },
        "cid" => { passport.cid = Some(String::from(v[1])) },
        _ => { eprintln!("Error occured while parsing entry.")}

      }
    }
    return passport;
  }

  let entries : Vec<&str> = data
    .split("\n\n") // split when there is a double line break
    .collect();

  // remove linebreaks within entries
  let entries_nolb = entries.into_iter().map(|e| { e.replace("\n", " ")});

  // parse each entry
  let passports : Vec<Passport> = entries_nolb.map(|e| { parse_entry(&e)}).collect();

  return passports
}

fn part_1() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let passports = parse_passport_data(&data);

  // count how many passports are valid
  let valid_count = passports.into_iter().fold(0, |acc, p| { return if p.validate1() { acc + 1 } else { acc } });

  println!("Valid passports: {}", valid_count);
}

fn part_2() {
  let data = fs::read_to_string(FILENAME)
    .unwrap();

  let passports = parse_passport_data(&data);

  // count how many passports are valid
  let valid_count = passports.into_iter().fold(0, |acc, p| { return if p.validate2() { acc + 1 } else { acc } });

  println!("Valid passports: {}", valid_count);
}

fn main() {
    let args = App::new("Advent of Code - Day 4")
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