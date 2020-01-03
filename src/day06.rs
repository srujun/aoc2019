use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::problem::Problem;

const COM: &str = "COM";
const YOU: &str = "YOU";
const SAN: &str = "SAN";

#[derive(Default)]
pub struct DaySix {}

impl DaySix {
  pub fn new() -> Self {
    Self {}
  }

  fn get_height_map<'a>(orbit_map: &HashMap<&'a str, &'a str>) -> HashMap<&'a str, usize> {
    let mut height_map = HashMap::new();
    height_map.insert(COM, 0);

    for obj in orbit_map.keys() {
      Self::get_and_set_height(obj, &mut height_map, orbit_map);
    }
    for obj in orbit_map.values() {
      Self::get_and_set_height(obj, &mut height_map, orbit_map);
    }

    height_map
  }

  fn get_and_set_height<'a>(
    object: &'a str,
    height_map: &mut HashMap<&'a str, usize>,
    orbit_map: &HashMap<&'a str, &'a str>,
  ) -> usize {
    match height_map.get(object) {
      Some(&height) => height,
      None => {
        let parent = orbit_map
          .get(object)
          .unwrap_or_else(|| panic!("Parent of {} not found!", object));
        let height = 1 + Self::get_and_set_height(parent, height_map, orbit_map);
        height_map.insert(object, height);
        height
      }
    }
  }
}

impl Problem for DaySix {
  fn soln_one(&self) -> Option<String> {
    Some("135690".to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let pattern = Regex::new(r"^([A-Z\d]+)\)([A-Z\d]+)$").unwrap();
    let orbits: Vec<(&str, &str)> = input
      .split('\n')
      .map(|line| pattern.captures(line).unwrap())
      .map(|caps| (caps.get(1).unwrap(), caps.get(2).unwrap()))
      .map(|(x, y)| (x.as_str(), y.as_str()))
      .collect();

    // ABC)XYZ == key:XYZ,value:ABC
    let mut orbit_map = HashMap::new();
    for (earth, moon) in &orbits {
      orbit_map.insert(*moon, *earth);
    }

    let height_map = Self::get_height_map(&orbit_map);
    let num_orbits: usize = height_map.values().sum();

    Some(num_orbits.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some("298".to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let pattern = Regex::new(r"^([A-Z\d]+)\)([A-Z\d]+)$").unwrap();
    let orbits: Vec<(&str, &str)> = input
      .split('\n')
      .map(|line| pattern.captures(line).unwrap())
      .map(|caps| (caps.get(1).unwrap(), caps.get(2).unwrap()))
      .map(|(x, y)| (x.as_str(), y.as_str()))
      .collect();

    // earth)moon == key:moon,value:earth
    let mut orbit_map = HashMap::new();
    for (earth, moon) in &orbits {
      orbit_map.insert(*moon, *earth);
    }

    // path from SAN to COM
    let mut santa_path = Vec::new();
    let mut santa_path_seen = HashSet::new();
    let mut curr = SAN;
    while let Some(parent) = orbit_map.get(curr) {
      curr = parent;
      santa_path.push(curr);
      santa_path_seen.insert(curr);
    }

    // Path from YOU to common ancestor
    let mut curr = YOU;
    let mut length = 0;
    while let Some(parent) = orbit_map.get(curr) {
      curr = parent;
      if santa_path_seen.contains(curr) {
        break;
      }
      length += 1;
    }

    // Add up SAN to common ancestor
    let common = curr;
    for curr in santa_path {
      if curr == common {
        break;
      }
      length += 1;
    }

    Some(length.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::DaySix;
  use crate::problem::Problem;

  #[test]
  fn part_one_case1() {
    let problem = DaySix::new();
    let input = "COM)B\n\
                 B)C\n\
                 C)D\n\
                 D)E\n\
                 E)F\n\
                 B)G\n\
                 G)H\n\
                 D)I\n\
                 E)J\n\
                 J)K\n\
                 K)L";
    assert_eq!(problem.part_one(input).unwrap(), "42");
  }

  #[test]
  fn part_two_case1() {
    let problem = DaySix::new();
    let input = "COM)B\n\
                 B)C\n\
                 C)D\n\
                 D)E\n\
                 E)F\n\
                 B)G\n\
                 G)H\n\
                 D)I\n\
                 E)J\n\
                 J)K\n\
                 K)L\n\
                 K)YOU\n\
                 I)SAN";
    assert_eq!(problem.part_two(input).unwrap(), "4");
  }
}
