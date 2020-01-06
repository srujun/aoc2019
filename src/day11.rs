use std::cmp::Ordering;
use std::collections::HashMap;

use crate::intcode::{self, Intcode};
use crate::problem::Problem;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn update(&mut self, dir: Direction) {
    match dir {
      Direction::Left => self.x -= 1,
      Direction::Up => self.y += 1,
      Direction::Right => self.x += 1,
      Direction::Down => self.y -= 1,
    };
  }
}

/// Ordering based on row-major index.
/// Since +Y is upwards, +Y is less than -Y.
impl Ord for Position {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.y > other.y {
      Ordering::Less
    } else if self.y < other.y {
      Ordering::Greater
    } else {
      self.x.cmp(&other.x)
    }
  }
}

/// Ordering based on row-major index.
/// Since +Y is upwards, +Y is less than -Y.
impl PartialOrd for Position {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
  Left,
  Up,
  Right,
  Down,
}

impl Direction {
  /// 0 => left 90 deg
  /// 1 => right 90 deg
  /// _ => panic
  fn turn(self, val: u8) -> Self {
    match self {
      Self::Left => match val {
        0 => Self::Down,
        1 => Self::Up,
        _ => panic!("Cannot turn {}!", val),
      },
      Self::Up => match val {
        0 => Self::Left,
        1 => Self::Right,
        _ => panic!("Cannot turn {}!", val),
      },
      Self::Right => match val {
        0 => Self::Up,
        1 => Self::Down,
        _ => panic!("Cannot turn {}!", val),
      },
      Self::Down => match val {
        0 => Self::Right,
        1 => Self::Left,
        _ => panic!("Cannot turn {}!", val),
      },
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum Color {
  Black,
  White,
}

impl Color {
  fn from(val: u8) -> Self {
    match val {
      0 => Color::Black,
      1 => Color::White,
      _ => panic!("No color for {}!", val),
    }
  }

  fn repr(self) -> &'static str {
    match self {
      Self::Black => ".",
      Self::White => "#",
    }
  }
}

const DEFAULT_COLOR: Color = Color::Black;

struct Robot {
  intcode: Intcode,
  curr_pos: Position,
  curr_dir: Direction,
}

impl Robot {
  fn new(program: Vec<i64>) -> Self {
    Self {
      intcode: Intcode::new(program.to_vec()),
      curr_pos: Position { x: 0, y: 0 },
      curr_dir: Direction::Up,
    }
  }

  /// Runs the robot.
  /// `painted` is a map of panel position -> painted color.
  fn run(&mut self, painted: &mut HashMap<Position, Color>) {
    while !self.intcode.has_halted {
      self
        .intcode
        .inputs
        .push(*painted.get(&self.curr_pos).unwrap_or(&DEFAULT_COLOR) as i64);
      self.intcode.run();
      let idx = self.intcode.outputs.len() - 2;
      let outputs = self.intcode.outputs.get(idx..).unwrap();

      // First output is color
      let color = Color::from(*outputs.get(0).unwrap() as u8);
      painted.insert(self.curr_pos, color);

      // Second output is movement
      self.curr_dir = self.curr_dir.turn(*outputs.get(1).unwrap() as u8);
      self.curr_pos.update(self.curr_dir);
    }
  }
}

#[derive(Default)]
pub struct DayEleven {
  debug: bool,
}

impl DayEleven {}

impl Problem for DayEleven {
  fn new() -> Self {
    Self { debug: false }
  }

  fn debug() -> Self {
    Self { debug: true }
  }

  fn soln_one(&self) -> Option<String> {
    Some("2418".to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let program: Vec<i64> = intcode::parse_program(input);
    let mut robot = Robot::new(program);

    let mut painted: HashMap<Position, Color> = HashMap::new();
    robot.run(&mut painted);

    Some(painted.len().to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let program: Vec<i64> = intcode::parse_program(input);
    let mut robot = Robot::new(program);

    let mut painted: HashMap<Position, Color> = HashMap::new();
    // Set the starting WHITE panel
    painted.insert(robot.curr_pos, Color::White);
    robot.run(&mut painted);

    let mut all_positions: Vec<&Position> = painted.keys().collect();
    all_positions.sort_unstable(); // unstable is faster

    let min_x: i32 = all_positions.iter().min_by_key(|pos| pos.x).unwrap().x;
    let max_x: i32 = all_positions.iter().max_by_key(|pos| pos.x).unwrap().x;
    let min_y: i32 = all_positions.iter().min_by_key(|pos| pos.y).unwrap().y;
    let max_y: i32 = all_positions.iter().max_by_key(|pos| pos.y).unwrap().y;
    if self.debug {
      println!("min_x: {}", min_x);
      println!("max_x: {}", max_x);
      println!("min_y: {}", min_y);
      println!("max_y: {}", max_y);
    }
    let mut pos_iter = all_positions.iter();
    let mut curr_pos = pos_iter.next();

    for y in (min_y..=max_y).rev() {
      // Have to reverse the range as Y axis is flipped
      for x in min_x..=max_x {
        if curr_pos.is_some() && curr_pos.unwrap().x == x && curr_pos.unwrap().y == y {
          if self.debug {
            print!("{}", painted.get(curr_pos.unwrap()).unwrap().repr());
          }
          curr_pos = pos_iter.next();
        } else if self.debug {
          print!("{}", DEFAULT_COLOR.repr());
        }
      }
      if self.debug {
        println!();
      }
    }

    Some("Done".to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::Position;

  #[test]
  fn position_vertical() {
    let p1 = Position { x: 0, y: 5 };
    let p2 = Position { x: 0, y: 7 };
    assert_eq!(p1 < p2, false);
    assert_eq!(p2 < p1, true);
  }

  #[test]
  fn position_horizontal() {
    let p1 = Position { x: 4, y: 3 };
    let p2 = Position { x: -1, y: 3 };
    assert_eq!(p1 < p2, false);
    assert_eq!(p2 < p1, true);
  }

  #[test]
  fn position_trailing_diag() {
    let p1 = Position { x: -1, y: 0 };
    let p2 = Position { x: 2, y: -3 };
    assert_eq!(p1 < p2, true);
    assert_eq!(p2 < p1, false);
  }

  #[test]
  fn position_leading_diag() {
    let p1 = Position { x: 4, y: 5 };
    let p2 = Position { x: 6, y: 10 };
    assert_eq!(p1 < p2, false);
    assert_eq!(p2 < p1, true);
  }
}
