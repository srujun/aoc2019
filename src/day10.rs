use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use std::slice;

use fraction::Decimal;

use crate::problem::Problem;

type NumVisible = usize;

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
struct Angle {
  angle: Decimal,
}

impl Angle {
  fn new(angle: Decimal) -> Self {
    Self { angle }
  }
}

impl Ord for Angle {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .angle
      .partial_cmp(&other.angle)
      .unwrap_or(Ordering::Equal)
  }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Asteroid(i16, i16);

impl Asteroid {
  /// Returns the angle in radians in range [0, 2*PI) going
  /// clockwise starting from the +Y axis as is given in the problem.
  fn angle(self, other: Self) -> Angle {
    let (dy, dx) = ((other.1 - self.1) as f32, (other.0 - self.0) as f32);
    // ((pi/2 - angle) + 4pi) % 2pi
    let radians = (FRAC_PI_2 - dy.atan2(dx) + 4_f32 * PI) % (2_f32 * PI);
    Angle::new(Decimal::from(radians))
  }

  fn distance(self, other: Self) -> f32 {
    let (dy, dx) = ((other.1 - self.1) as f32, (other.0 - self.0) as f32);
    dy.hypot(dx)
  }
}

#[derive(Default)]
pub struct DayTen {}

impl DayTen {
  /// The problem statement uses a flipped Y axis (+Y is downwards).
  /// This function flips the Y coordinates.
  fn parse_asteroids(input: &str) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    for (y, row) in input.split('\n').enumerate() {
      for (x, val) in row.chars().enumerate() {
        match val {
          '.' => {}
          '#' => {
            asteroids.push(Asteroid(x as i16, -(y as i16)));
          }
          _ => panic!("Unexpected char {}", val),
        }
      }
    }
    asteroids
  }

  fn best_position(asteroids: &[Asteroid]) -> (Asteroid, NumVisible) {
    // position -> num asteroids visible
    let mut visible: HashMap<Asteroid, NumVisible> = HashMap::with_capacity(asteroids.len());

    for &a1 in asteroids.iter() {
      let mut uniq_angles: HashSet<Angle> = HashSet::with_capacity(asteroids.len());
      for &a2 in asteroids.iter() {
        if a1 == a2 {
          // skip the same asteroid
          continue;
        }
        let angle = a1.angle(a2);
        uniq_angles.insert(angle);
      }
      visible.insert(a1, uniq_angles.len());
    }

    let (best, num) = visible.iter().max_by_key(|(_, &num)| num).unwrap();
    (*best, *num)
  }
}

impl Problem for DayTen {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some("329".to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let asteroids = DayTen::parse_asteroids(input);
    let (_, num_visible) = DayTen::best_position(&asteroids);
    Some(num_visible.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some("512".to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let asteroids = DayTen::parse_asteroids(input);
    let (best_asteroid, _) = DayTen::best_position(&asteroids);

    // sorted angle -> asteroids at that angle
    let mut at_angle: BTreeMap<Angle, Vec<Asteroid>> = BTreeMap::new();
    // asteroid -> distance to best_asteroid
    let mut distances: HashMap<Asteroid, f32> = HashMap::with_capacity(asteroids.len() - 1);

    for &asteroid in asteroids.iter() {
      if asteroid == best_asteroid {
        continue; // skip the same asteroid
      }
      let angle = best_asteroid.angle(asteroid);
      let distance = best_asteroid.distance(asteroid);
      at_angle
        .entry(angle)
        .and_modify(|vec| vec.push(asteroid))
        .or_insert_with(|| vec![asteroid]);
      distances.insert(asteroid, distance);
    }

    let num_angles = at_angle.keys().len();

    // sort the asteroids by their distance.
    at_angle.values_mut().for_each(|vec| {
      vec.sort_by(|a, b| f32_cmp(*distances.get(a).unwrap(), *distances.get(b).unwrap()))
    });

    // convert each vec to an iterator
    let mut at_angle_iter: BTreeMap<Angle, slice::IterMut<Asteroid>> = at_angle
      .iter_mut()
      .map(|(key, val)| (*key, val.iter_mut()))
      .collect();

    // consume each angle's iterator, cycling through the ordered angle keys
    let mut curr: &Asteroid = &Asteroid(0, 0);
    for num in 0..200 {
      curr = at_angle_iter
        .iter_mut()
        .nth(num % num_angles)
        .unwrap()
        .1
        .next()
        .unwrap();
      // println!("{} => ({},{})", num + 1, curr.0, -curr.1);
    }

    Some((curr.0 * 100 - curr.1).to_string())
  }
}

fn f32_cmp(a: f32, b: f32) -> Ordering {
  a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

#[cfg(test)]
mod tests {
  use super::DayTen;
  use crate::problem::Problem;

  #[test]
  fn part_one_given() {
    let problem = DayTen::new();
    let input = ".#..#\n\
                 .....\n\
                 #####\n\
                 ....#\n\
                 ...##";
    // 3,4
    assert_eq!(problem.part_one(input).unwrap(), "8".to_string());
  }

  #[test]
  fn part_one_case1() {
    let problem = DayTen::new();
    let input = "......#.#.\n\
                 #..#.#....\n\
                 ..#######.\n\
                 .#.#.###..\n\
                 .#..#.....\n\
                 ..#....#.#\n\
                 #..#....#.\n\
                 .##.#..###\n\
                 ##...#..#.\n\
                 .#....####";
    // 5,8
    assert_eq!(problem.part_one(input).unwrap(), "33".to_string());
  }

  #[test]
  fn part_one_case2() {
    let problem = DayTen::new();
    let input = "#.#...#.#.\n\
                 .###....#.\n\
                 .#....#...\n\
                 ##.#.#.#.#\n\
                 ....#.#.#.\n\
                 .##..###.#\n\
                 ..#...##..\n\
                 ..##....##\n\
                 ......#...\n\
                 .####.###.";
    // 1,2
    assert_eq!(problem.part_one(input).unwrap(), "35".to_string());
  }

  #[test]
  fn part_one_case3() {
    let problem = DayTen::new();
    let input = ".#..#..###\n\
                 ####.###.#\n\
                 ....###.#.\n\
                 ..###.##.#\n\
                 ##.##.#.#.\n\
                 ....###..#\n\
                 ..#.#..#.#\n\
                 #..#.#.###\n\
                 .##...##.#\n\
                 .....#.#..";
    // 6,3
    assert_eq!(problem.part_one(input).unwrap(), "41".to_string());
  }

  #[test]
  fn part_one_case4() {
    let problem = DayTen::new();
    let input = ".#..##.###...#######\n\
                 ##.############..##.\n\
                 .#.######.########.#\n\
                 .###.#######.####.#.\n\
                 #####.##.#.##.###.##\n\
                 ..#####..#.#########\n\
                 ####################\n\
                 #.####....###.#.#.##\n\
                 ##.#################\n\
                 #####.##.###..####..\n\
                 ..######..##.#######\n\
                 ####.##.####...##..#\n\
                 .#####..#.######.###\n\
                 ##...#.##########...\n\
                 #.##########.#######\n\
                 .####.#.###.###.#.##\n\
                 ....##.##.###..#####\n\
                 .#.#.###########.###\n\
                 #.#.#.#####.####.###\n\
                 ###.##.####.##.#..##";
    // 11,13
    assert_eq!(problem.part_one(input).unwrap(), "210".to_string());
  }

  #[test]
  fn part_two_case1() {
    let problem = DayTen::new();
    let input = ".#..##.###...#######\n\
                 ##.############..##.\n\
                 .#.######.########.#\n\
                 .###.#######.####.#.\n\
                 #####.##.#.##.###.##\n\
                 ..#####..#.#########\n\
                 ####################\n\
                 #.####....###.#.#.##\n\
                 ##.#################\n\
                 #####.##.###..####..\n\
                 ..######..##.#######\n\
                 ####.##.####...##..#\n\
                 .#####..#.######.###\n\
                 ##...#.##########...\n\
                 #.##########.#######\n\
                 .####.#.###.###.#.##\n\
                 ....##.##.###..#####\n\
                 .#.#.###########.###\n\
                 #.#.#.#####.####.###\n\
                 ###.##.####.##.#..##";
    // 11,13
    assert_eq!(problem.part_two(input).unwrap(), "802".to_string());
  }
}
