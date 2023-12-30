use core::num;
use std::fs;

pub fn day2() {
  let file_path = "C:/Users/Daniel/Projects/Programming/advent_of_code_2023/input/day2";
  let contents: String = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");
  let iterator = contents.split("\n").enumerate();
  let mut total = 0;
  for (game, line) in iterator {
    // println!("{}", line);
    let game_pulls = line.split(":").nth(1).unwrap();
    let pulls = game_pulls.split(";");
    let mut is_possible = true;
    for pull in pulls {
      let cubes_list = pull.split(",");
      for cubes in cubes_list {
        let mut cube = cubes.trim().split(" ");
        let number_str = cube.next().unwrap();
        let num = number_str.parse::<usize>().unwrap();
        let cur: Cubes = Cubes { number: num,  color: str_to_enum(cube.next().unwrap()).unwrap() };
        // println!("{:?}", cur);
        if !cur.is_smaller() {
          is_possible = false;
        }

      }
    }
    if is_possible {
      total += game + 1;
    }
    println!("{} {}", total, line);



    // println!("{}", game_pulls)
  }

}

pub fn day2_pt2() {
  let file_path = "C:/Users/Daniel/Projects/Programming/advent_of_code_2023/input/day2";
  let contents: String = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");
  let iterator = contents.split("\n").enumerate();
  let mut sum = 0;
  for (game, line) in iterator {
    // println!("{}", line);
    let game_pulls = line.split(":").nth(1).unwrap();
    let pulls: std::str::Split<'_, &str> = game_pulls.split(";");
    let mut red_max: Cubes = Cubes {..Default::default()};
    let mut green_max: Cubes = Cubes {..Default::default()};
    let mut blue_max: Cubes = Cubes {..Default::default()};
    for pull in pulls {
      let cubes_list = pull.split(",");
      for cubes in cubes_list {
        let mut cube = cubes.trim().split(" ");
        let number_str = cube.next().unwrap();
        let num = number_str.parse::<usize>().unwrap();
        let cur: Cubes = Cubes { number: num,  color: str_to_enum(cube.next().unwrap()).unwrap() };
        match cur.color {
          CubeColor::Blue => {
            blue_max = blue_max.max(cur);
          },
          CubeColor::Green => {
            green_max = green_max.max(cur);
          },
          CubeColor::Red => {
            red_max = red_max.max(cur);
          },
        }
        // println!("{:?}", cur);
      }
    }
    sum += red_max.number * blue_max.number * green_max.number;
  }
  println!("{}", sum);

}



fn str_to_enum(text: &str) -> Option<CubeColor> {
  match text {
    "green" => Some(CubeColor::Green),
    "blue" => Some(CubeColor::Blue),
    "red" => Some(CubeColor::Red),
    _ => None
  }
}




#[derive(Debug, Default)]
struct Cubes {
  number: usize,
  color: CubeColor,
}

impl Cubes {
  pub fn is_smaller(self) -> bool {
    match self {
      Cubes { color : CubeColor::Green, ..} => self.number <= 13,
      Cubes { color : CubeColor::Blue, ..} => self.number <= 14,
      Cubes { color : CubeColor::Red, ..} => self.number <= 12,
    }
  }

  pub fn max(self, other: Cubes) -> Cubes {
    if self.number > other.number {
      self
    } else {
      other
    }
  }
}



#[derive(Debug, Default)]
enum CubeColor {
  #[default] Green = 0,
  Blue = 1,
  Red = 2,
}