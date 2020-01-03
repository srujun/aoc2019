use std::cmp;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::u32;

use crate::problem::Problem;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
  x: i32,
  y: i32,
}

impl Coord {
  fn new() -> Self {
    Coord { x: 0, y: 0 }
  }

  fn from_vals(x: i32, y: i32) -> Self {
    Coord { x, y }
  }

  fn manhattan(self) -> u32 {
    self.x.abs() as u32 + self.y.abs() as u32
  }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
  Left,
  Right,
  Up,
  Down,
}

#[derive(Clone, Debug)]
struct Movement {
  dir: Direction,
  amount: usize,
}

impl Movement {
  fn parse(input: &str) -> Self {
    let amount = input
      .get(1..)
      .unwrap()
      .parse::<usize>()
      .expect("Unknown movement value!");
    match input.chars().nth(0) {
      Some('L') => Self {
        dir: Direction::Left,
        amount,
      },
      Some('R') => Self {
        dir: Direction::Right,
        amount,
      },
      Some('U') => Self {
        dir: Direction::Up,
        amount,
      },
      Some('D') => Self {
        dir: Direction::Down,
        amount,
      },
      _ => panic!("Unknown movement!"),
    }
  }
}

struct Segment {
  steps_until: u32,
  dir: Direction,
  // The position of this segment on the fixed axis.
  // fixed_axis: i32,
  // The start and end of the segment on the other axis.
  bounds: RangeInclusive<i32>,
}

// Maps a fixed axis value to the list of wire segments along that axis.
type WireMap = BTreeMap<i32, Vec<Segment>>;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Intersection {
  wire1_steps: u32,
  wire2_steps: u32,
  point: Coord,
}

#[derive(Default)]
pub struct DayThree {}

impl DayThree {
  pub fn new() -> Self {
    Self {}
  }

  fn build_maps(horizontals: &mut WireMap, verticals: &mut WireMap, wire: &[Movement]) {
    let mut curr_pos = Coord::new();
    let mut steps: u32 = 0;
    for movement in wire {
      match movement.dir {
        Direction::Left => {
          horizontals.entry(curr_pos.y).or_default();
          let end = curr_pos.x;
          curr_pos.x -= movement.amount as i32;
          let start = curr_pos.x;
          let segment = Segment {
            steps_until: steps,
            dir: movement.dir,
            // fixed_axis: curr_pos.y,
            bounds: start..=end,
          };
          horizontals
            .entry(curr_pos.y)
            .and_modify(|w| w.push(segment));
        }
        Direction::Right => {
          horizontals.entry(curr_pos.y).or_default();
          let start = curr_pos.x;
          curr_pos.x += movement.amount as i32;
          let end = curr_pos.x;
          let segment = Segment {
            steps_until: steps,
            dir: movement.dir,
            // fixed_axis: curr_pos.y,
            bounds: start..=end,
          };
          horizontals
            .entry(curr_pos.y)
            .and_modify(|w| w.push(segment));
        }
        Direction::Up => {
          verticals.entry(curr_pos.x).or_default();
          let start = curr_pos.y;
          curr_pos.y += movement.amount as i32;
          let end = curr_pos.y;
          let segment = Segment {
            steps_until: steps,
            dir: movement.dir,
            // fixed_axis: curr_pos.x,
            bounds: start..=end,
          };
          verticals.entry(curr_pos.x).and_modify(|w| w.push(segment));
        }
        Direction::Down => {
          verticals.entry(curr_pos.x).or_default();
          let end = curr_pos.y;
          curr_pos.y -= movement.amount as i32;
          let start = curr_pos.y;
          let segment = Segment {
            steps_until: steps,
            dir: movement.dir,
            // fixed_axis: curr_pos.x,
            bounds: start..=end,
          };
          verticals.entry(curr_pos.x).and_modify(|w| w.push(segment));
        }
      }
      steps += movement.amount as u32;
    }
  }

  fn get_intersections(
    horizontals: &WireMap,
    verticals: &WireMap,
    other_wire: &[Movement],
  ) -> HashSet<Intersection> {
    let mut intersections: HashSet<Intersection> = HashSet::new();
    let mut curr_pos = Coord::new();
    let mut other_wire_steps = 0;

    for movement in other_wire {
      match movement.dir {
        Direction::Left => {
          let end = curr_pos.x;
          curr_pos.x -= movement.amount as i32;
          let start = curr_pos.x;
          for (xpos, segments) in verticals.range(start..=end) {
            for segment in segments {
              if segment.bounds.contains(&&curr_pos.y) {
                let wire1_diff = match segment.dir {
                  Direction::Up => (curr_pos.y - segment.bounds.start()).abs(),
                  Direction::Down => (curr_pos.y - segment.bounds.end()).abs(),
                  _ => panic!("Unexpected dir in verticals"),
                };
                let isec = Intersection {
                  wire1_steps: segment.steps_until + wire1_diff as u32,
                  wire2_steps: other_wire_steps + (xpos - end).abs() as u32,
                  point: Coord::from_vals(*xpos, curr_pos.y),
                };
                intersections.insert(isec);
              }
            }
          }
        }
        Direction::Right => {
          let start = curr_pos.x;
          curr_pos.x += movement.amount as i32;
          let end = curr_pos.x;
          for (xpos, segments) in verticals.range(start..=end) {
            for segment in segments {
              if segment.bounds.contains(&&curr_pos.y) {
                let wire1_diff = match segment.dir {
                  Direction::Up => (curr_pos.y - segment.bounds.start()).abs(),
                  Direction::Down => (curr_pos.y - segment.bounds.end()).abs(),
                  _ => panic!("Unexpected dir in verticals"),
                };
                let isec = Intersection {
                  wire1_steps: segment.steps_until + wire1_diff as u32,
                  wire2_steps: other_wire_steps + (xpos - start).abs() as u32,
                  point: Coord::from_vals(*xpos, curr_pos.y),
                };
                intersections.insert(isec);
              }
            }
          }
        }
        Direction::Up => {
          let start = curr_pos.y;
          curr_pos.y += movement.amount as i32;
          let end = curr_pos.y;
          for (ypos, segments) in horizontals.range(start..=end) {
            for segment in segments {
              if segment.bounds.contains(&&curr_pos.x) {
                let wire1_diff = match segment.dir {
                  Direction::Left => (curr_pos.x - segment.bounds.end()).abs(),
                  Direction::Right => (curr_pos.x - segment.bounds.start()).abs(),
                  _ => panic!("Unexpected dir in horizontals"),
                };
                let isec = Intersection {
                  wire1_steps: segment.steps_until + wire1_diff as u32,
                  wire2_steps: other_wire_steps + (ypos - start).abs() as u32,
                  point: Coord::from_vals(curr_pos.x, *ypos),
                };
                intersections.insert(isec);
              }
            }
          }
        }
        Direction::Down => {
          let end = curr_pos.y;
          curr_pos.y -= movement.amount as i32;
          let start = curr_pos.y;
          for (ypos, segments) in horizontals.range(start..=end) {
            for segment in segments {
              if segment.bounds.contains(&&curr_pos.x) {
                let wire1_diff = match segment.dir {
                  Direction::Left => (curr_pos.x - segment.bounds.end()).abs(),
                  Direction::Right => (curr_pos.x - segment.bounds.start()).abs(),
                  _ => panic!("Unexpected dir in horizontals"),
                };
                let isec = Intersection {
                  wire1_steps: segment.steps_until + wire1_diff as u32,
                  wire2_steps: other_wire_steps + (ypos - end).abs() as u32,
                  point: Coord::from_vals(curr_pos.x, *ypos),
                };
                intersections.insert(isec);
              }
            }
          }
        }
      }

      other_wire_steps += movement.amount as u32;
    }
    // println!("Intersections: {:?}", intersections);
    intersections
  }
}

impl Problem for DayThree {
  fn soln_one(&self) -> Option<String> {
    Some("5357".to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let mut wires = input.split('\n');
    let wire1: Vec<Movement> = wires
      .next()
      .unwrap()
      .split(',')
      .map(Movement::parse)
      .collect();
    let wire2: Vec<Movement> = wires
      .next()
      .unwrap()
      .split(',')
      .map(Movement::parse)
      .collect();

    // ypos to list of wires in x-axis range
    let mut horizontals: WireMap = BTreeMap::new();
    // xpos to list of wires in y-axis range
    let mut verticals: WireMap = BTreeMap::new();

    // Populate the WireMaps
    Self::build_maps(&mut horizontals, &mut verticals, &wire1);

    // println!("Horizontals: {:?}", horizontals);
    // println!("Verticals: {:?}", verticals);

    // find the closest intersection by manhattan distance
    Some(
      Self::get_intersections(&horizontals, &verticals, &wire2)
        .iter()
        .map(|isec| isec.point)
        .map(|coord| coord.manhattan())
        .filter(|&x| x > 0)
        .fold(u32::MAX, cmp::min)
        .to_string(),
    )
  }

  fn soln_two(&self) -> Option<String> {
    Some("101956".to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let mut wires = input.split('\n');
    let wire1: Vec<Movement> = wires
      .next()
      .unwrap()
      .split(',')
      .map(Movement::parse)
      .collect();
    let wire2: Vec<Movement> = wires
      .next()
      .unwrap()
      .split(',')
      .map(Movement::parse)
      .collect();

    // ypos to list of wires in x-axis range
    let mut horizontals: WireMap = BTreeMap::new();
    // xpos to list of wires in y-axis range
    let mut verticals: WireMap = BTreeMap::new();

    // Populate the WireMaps
    Self::build_maps(&mut horizontals, &mut verticals, &wire1);

    Some(
      Self::get_intersections(&horizontals, &verticals, &wire2)
        .iter()
        .map(|isec| isec.wire1_steps + isec.wire2_steps)
        .filter(|&x| x > 0)
        .fold(u32::MAX, cmp::min)
        .to_string(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::super::problem::Problem;
  use super::DayThree;

  #[test]
  fn part_one_given() {
    let problem = DayThree {};
    let answer = problem.part_one(
      "R8,U5,L5,D3\n\
       U7,R6,D4,L4",
    );
    assert_eq!(answer.unwrap(), "6");
  }

  #[test]
  fn part_one_case1() {
    let problem = DayThree {};
    let answer = problem.part_one(
      "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
       U62,R66,U55,R34,D71,R55,D58,R83",
    );
    assert_eq!(answer.unwrap(), "159");
  }

  #[test]
  fn part_one_case2() {
    let problem = DayThree {};
    let answer = problem.part_one(
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
       U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );
    assert_eq!(answer.unwrap(), "135");
  }

  #[test]
  fn part_two_given() {
    let problem = DayThree {};
    let answer = problem.part_two(
      "R8,U5,L5,D3\n\
       U7,R6,D4,L4",
    );
    assert_eq!(answer.unwrap(), "30");
  }

  #[test]
  fn part_two_case1() {
    let problem = DayThree {};
    let answer = problem.part_two(
      "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
       U62,R66,U55,R34,D71,R55,D58,R83",
    );
    assert_eq!(answer.unwrap(), "610");
  }

  #[test]
  fn part_two_case2() {
    let problem = DayThree {};
    let answer = problem.part_two(
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
       U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );
    assert_eq!(answer.unwrap(), "410");
  }
}
