use std::fs;
use clap::{Arg, App};
use regex::Regex;

const FILENAME: &str = "input.txt";

struct TicketField {
    name: String,
    ranges: Vec<u32>
}

type Ticket = Vec<u32>;

fn parse_data(input: &String) -> (Vec<TicketField>, Ticket, Vec<Ticket>) {
  let mut input_parts = input.split("\n\n");
  let fields: Vec<TicketField> = input_parts.next().unwrap().lines().map(|l| {
    let re = Regex::new(r"^([a-z\s]+):\s([0-9]+)-([0-9]+)\sor\s([0-9]+)-([0-9]+)").unwrap();
    let matches: Vec<&str> = re.captures(l).unwrap().iter().map(|m| { m.unwrap().as_str() }).collect();
    return TicketField{
      name: matches[1].to_string(),
      ranges: matches[2..].iter().map(|s| { s.parse().unwrap() }).collect()
    }
  }).collect();

  let your_ticket: Ticket = input_parts.next().unwrap().lines().nth(1).unwrap().split(",").map(|s| {
    s.parse::<u32>().unwrap()
  }).collect();

  let nearby_tickets : Vec<Ticket> = input_parts.next().unwrap().lines().skip(1).map(|l| {
    return l.split(",").map(|s| {
      s.parse::<u32>().unwrap()
    }).collect::<Ticket>()
  }).collect();

  return (fields, your_ticket, nearby_tickets);
}

fn part_1(input: &String) -> u64 {

  let (fields, _, nearby_tickets) = parse_data(input);
  
  let all_fields = fields.iter().fold(vec![] as Vec<u32>, |mut acc, f| {
    acc.extend(&f.ranges);
    return acc;
  });  

  let errors: Vec<u32> = nearby_tickets.iter().map(|t| {
    return t.iter().fold(vec![], |mut acc, &value| {
      let valid = all_fields.chunks(2).any(|f| {
        return value >= f[0] && value <= f[1]
      });
      if !valid { acc.push(value) }
      
      return acc;
    });
  }).flatten().collect();

  return errors.iter().sum::<u32>() as u64;
}

fn part_2(input: &String) -> u64 {

  fn is_ticket_valid(ticket: &Ticket, fields: &Vec<u32>) -> bool {
    return ticket.iter().any(|&value| {
      let valid = fields.chunks(2).any(|f| {
        return value >= f[0] && value <= f[1];
      });
      return !valid;
    }) == false
  }

  let (fields, your_ticket, nearby_tickets) = parse_data(input);

  let all_fields = fields.iter().fold(vec![] as Vec<u32>, |mut acc, f| {
    acc.extend(&f.ranges);
    return acc;
  });

  let valid_tickets: Vec<Ticket> = nearby_tickets.into_iter().filter(|t| {
    return is_ticket_valid(t, &all_fields);
  }).collect();

  // find out possible indices for fields
  let mut field_candidates: Vec<(String,Vec<usize>)> = fields.iter().map(|f| {
    let mut candidates = vec![true; fields.len()];
    for ticket in &valid_tickets {
      for (i, &value) in ticket.iter().enumerate() {
        let in_range = value >= f.ranges[0] && value <= f.ranges[1] 
          || value >= f.ranges[2] && value <= f.ranges[3];
        if !in_range { candidates[i] = false; }
      }
    }
    return (
      f.name.to_string(), 
      candidates.iter().enumerate().fold(vec![],|mut acc, (i,&c)| {
        if c == true { acc.push(i) }
        return acc;
      })
    );
  }).collect();

  // sort possible candidates
  field_candidates.sort_by(|a,b| { a.1.len().cmp(&b.1.len()) });

  // filter candidate indices by previous candidates indices
  field_candidates = [vec![(String::from(""),vec![])], field_candidates].concat()
    .windows(2).map(|cs| {
      let filtered_indices: Vec<usize> = cs[1].1.clone().into_iter().filter(|&v| {
        return !cs[0].1.iter().any(|&w| { v == w })
      }).collect();
      return(
        cs[1].0.to_string(),
        filtered_indices
      );
    }).collect();


  // find out fields starting with departure
  let departure_fields: Vec<(String,Vec<usize>)> = field_candidates.into_iter().filter(|f| {
    return f.0.starts_with("departure");
  }).collect();

  // multiply all departure fields from your ticket
  let product = departure_fields.iter().fold(1, |acc,c| { 
    acc * your_ticket[c.1[0]] as u64
  });

  return product;
}

fn main() {
  let args = App::new("Advent of Code - Day 16")
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