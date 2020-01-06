use crate::problem::Problem;

#[derive(Default)]
pub struct DayOne {}

impl DayOne {
  fn calc_fuel(mass: u32) -> i32 {
    (mass as f32 / 3.).floor() as i32 - 2
  }

  fn fuel(masses: &[u32]) -> u32 {
    masses
      .iter()
      .map(|&m| Self::calc_fuel(m))
      .filter(|f| f.is_positive())
      .sum::<i32>() as u32
  }
}

impl Problem for DayOne {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some("3412496".to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let masses: Vec<u32> = input
      .split('\n')
      .map(|s| s.parse())
      .filter_map(Result::ok)
      .collect();
    Some(Self::fuel(&masses).to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some("5115845".to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let mut total_fuel = 0;
    for mass_str in input.split('\n') {
      let mass: u32 = mass_str.parse().unwrap();
      let mut fuel: i32 = Self::calc_fuel(mass);
      if fuel > 0 {
        total_fuel += fuel;
      }
      loop {
        fuel = Self::calc_fuel(fuel as u32);
        if fuel <= 0 {
          break;
        }
        total_fuel += fuel;
      }
    }
    Some(total_fuel.to_string())
  }
}

#[cfg(test)]
mod tests {}
