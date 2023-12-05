use std::fs;
use regex::Regex;

pub fn day1() {
  let file_path = "C:/Users/Daniel/Projects/Programming/advent_of_code_2023/input/day1";

  let contents: String = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let iterator = contents.split("\n");

  let mut total = 0;
  for line in iterator {
    let mut pt1: usize = 0;
    let mut pt2: usize = line.len() - 1;
    while !char::is_numeric(line.chars().nth(pt1).unwrap()) {
      pt1 += 1;
    }
    while !char::is_numeric(line.chars().nth(pt2).unwrap()) {
      pt2 -= 1;
    }
    total += line.chars().nth(pt1).unwrap().to_digit(10).unwrap() * 10;
    total += line.chars().nth(pt2).unwrap()
      .to_digit(10)
      .unwrap();
    println!("{}, {}, {}", pt1,  total, line);
  }
}

pub fn day1_pt2() {
  let file_path = "C:/Users/Daniel/Projects/Programming/advent_of_code_2023/input/day1";

  let contents: String = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let iterator = contents.split("\n");

  let mut total = 0;
  for line in iterator {
    let line_reverse = line.clone().chars().rev().collect::<String>();
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[1-9]").unwrap();
    let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
    let mut iter = matches.iter();
    let first = iter.next().unwrap();
    if first.len() != 1 {
      total += number_to_usize(first).unwrap() * 10;
    } else {
      total += first.chars()
      .next().unwrap()
      .to_digit(10).unwrap() as usize * 10;
    }

    let re_reverse = Regex::new(&r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[1-9]").unwrap();
    let matches_reverse: Vec<&str> = re_reverse.find_iter(line_reverse.as_str()).map(|m| m.as_str()).collect();
    let mut iter_reverse = matches_reverse.iter();
    let last = iter_reverse.next().unwrap();
    if last.len() != 1 {
      total += number_to_usize(last.chars().rev().collect::<String>().as_str()).unwrap();
    } else {
      total += last.chars()
      .next().unwrap()
      .to_digit(10).unwrap() as usize;
  }
  }
  println!("{}", total);
}

fn number_to_usize(number: &str) -> Option<usize> {
  match number {
    "one" => Some(1),
    "two" => Some(2),
    "three" => Some(3),
    "four" => Some(4),
    "five" => Some(5),
    "six" => Some(6),
    "seven" => Some(7),
    "eight" => Some(8),
    "nine" => Some(9),
    _ => None,
  }
}