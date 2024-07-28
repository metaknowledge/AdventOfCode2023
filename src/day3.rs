use std::fs;
use regex::Regex;

pub fn day3() {
  let file_path = "C:/Users/Daniel/Projects/Programming/advent_of_code_2023/input/day3";

  let contents: String = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  let lines = contents.split("\n");

  let mut matrix: Vec<Vec<char>> = Vec::new();
  for (y, line) in contents.lines().enumerate() {
    for (x, letter) in line.chars().enumerate() {
      match matrix.get_mut(y) {
          Some(vec) => vec.push(letter),
          None => matrix.push(vec![letter]),
      };
    }
  }
  println!("{:?}", matrix);
  let mut positions: Vec<NumberPositions> = Vec::new();
  for (y, line) in contents.lines().enumerate() {
    for (x, letter) in line.chars().enumerate() {
      if letter.is_ascii_digit() {
        if x == 0 {
          positions.push(check_next_digits(x, y, line));
        } else {
          match line.chars().nth(x-1) {
            Some(digit) => {
              if !digit.is_ascii_digit() {
                positions.push(check_next_digits(x, y, line));
              }
            },
            None => (),
          }
        }
      }
    }
  }
  println!("{:?}", positions);

  for position in positions {

  }


  // let expression = r"([0-9])+[^.0-9\n]";
  // let re = Regex::new(expression).unwrap();
  // let matches: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();
  // let mut iter = matches.iter();
  // println!("{:?}", iter);
  // let mut sum = 0;
  // for num in iter {
    //   let mut number = num.to_string();
    //   number.pop();
    //   let num = number.parse::<usize>().unwrap();
    //   sum += num;
    // }
    // println!("{}", sum)
}

fn get_surrounding_char(matrix: Vec<Vec<char>>, pos: NumberPositions) {
  let topleft = (pos.x as i32 - 1, pos.y as i32 +1);
  let bottomright = (pos.x as i32 + 1, pos.y as i32 - 1);


}

pub fn check_next_digits(x: usize, y: usize, line: &str) -> NumberPositions {
  let mut position = NumberPositions { x: x, y: y, ..Default::default()};
  match (line.chars().nth(x+1), line.chars().nth(x+2)) {
    (Some(_), Some(_)) => {
      position.len = 3;
    },
    (Some(_), None) => {
      position.len = 2;
    },
    _ => {
      position.len = 1;
    },
  }
  position
}

#[derive(Default, Debug)]
pub struct NumberPositions {
  x: usize,
  y: usize,
  len: usize,
}