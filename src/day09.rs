use crate::intcode::{self, Intcode};
use crate::problem::Problem;

#[derive(Default)]
pub struct DayNine {}

impl DayNine {
  pub fn new() -> Self {
    Self {}
  }
}

impl Problem for DayNine {
  fn soln_one(&self) -> Option<String> {
    Some("2662308295".to_string())
  }

  fn part_one(&self, program: &str) -> Option<String> {
    let amp_program: Vec<i64> = intcode::parse_program(program);
    let mut intcode = Intcode::new(amp_program.to_vec());
    intcode.inputs.push(1); // test mode
    intcode.run();

    intcode.outputs.last().map(|x| x.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some("63441".to_string())
  }

  fn part_two(&self, program: &str) -> Option<String> {
    let amp_program: Vec<i64> = intcode::parse_program(program);
    let mut intcode = Intcode::new(amp_program.to_vec());
    intcode.inputs.push(2); // sensor boost mode
    intcode.run();

    intcode.outputs.last().map(|x| x.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::DayNine;
  use crate::intcode::{self, Intcode};
  use crate::problem::Problem;

  #[test]
  fn part_one_case1() {
    let input: Vec<i64> = vec![
      109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let program = intcode::parse_program(
      &input
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(","),
    );
    let mut intcode = Intcode::new(program);
    intcode.run();
    assert_eq!(intcode.outputs, input);
  }

  #[test]
  fn part_one_case2() {
    let problem = DayNine::new();
    let output = problem
      .part_one("1102,34915192,34915192,7,4,7,99,0")
      .unwrap();
    assert_eq!(output.chars().count(), 16);
  }

  #[test]
  fn part_one_case3() {
    let problem = DayNine::new();
    assert_eq!(
      problem.part_one("104,1125899906842624,99").unwrap(),
      "1125899906842624"
    );
  }
}
