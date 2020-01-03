use crate::intcode::{self, Intcode};
use crate::problem::Problem;

#[derive(Default)]
pub struct DayFive {
  inputs: Vec<i32>,
}

impl DayFive {
  pub fn new() -> Self {
    Self { inputs: Vec::new() }
  }
}

impl Problem for DayFive {
  fn soln_one(&self) -> String {
    "7839346".to_string()
  }

  fn part_one(&self, program: &str) -> String {
    let program: Vec<i32> = intcode::parse_program(program);

    let inputs = if self.inputs.is_empty() {
      vec![1]
    } else {
      self.inputs.clone()
    };

    let mut intcode = Intcode::new(program);
    intcode.inputs = inputs;
    intcode.run();

    intcode
      .outputs
      .last()
      .map(i32::to_string)
      .unwrap_or_else(|| "".to_string())
  }

  fn soln_two(&self) -> String {
    "447803".to_string()
  }

  fn part_two(&self, program: &str) -> String {
    let program: Vec<i32> = intcode::parse_program(program);

    let inputs = if self.inputs.is_empty() {
      vec![5]
    } else {
      self.inputs.clone()
    };

    let mut intcode = Intcode::new(program);
    intcode.inputs = inputs;
    intcode.run();

    intcode
      .outputs
      .last()
      .map(i32::to_string)
      .unwrap_or_else(|| "".to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::DayFive;
  use crate::problem::Problem;

  #[test]
  fn part_one_case1() {
    let problem = DayFive { inputs: vec![100] };
    assert_eq!(problem.part_one("3,0,4,0,99"), "100");
  }

  #[test]
  fn part_one_case2() {
    let problem = DayFive { inputs: vec![] };
    assert_eq!(problem.part_one("1002,4,3,4,33"), "");
  }

  #[test]
  fn part_two_equal_to_8_pos() {
    let problem = DayFive { inputs: vec![10] };
    assert_eq!(problem.part_two("3,9,8,9,10,9,4,9,99,-1,8"), "0");
  }

  #[test]
  fn part_two_less_than_8_pos() {
    let problem = DayFive { inputs: vec![5] };
    assert_eq!(problem.part_two("3,9,7,9,10,9,4,9,99,-1,8"), "1");
  }

  #[test]
  fn part_two_equal_8_imm() {
    let problem = DayFive { inputs: vec![8] };
    assert_eq!(problem.part_two("3,3,1108,-1,8,3,4,3,99"), "1");
  }

  #[test]
  fn part_two_less_than_8_imm() {
    let problem = DayFive { inputs: vec![100] };
    assert_eq!(problem.part_two("3,3,1107,-1,8,3,4,3,99"), "0");
  }

  #[test]
  fn part_two_jump_pos_zero() {
    let problem = DayFive { inputs: vec![0] };
    assert_eq!(
      problem.part_two("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
      "0"
    );
  }

  #[test]
  fn part_two_jump_pos_nonzero() {
    let problem = DayFive { inputs: vec![12] };
    assert_eq!(
      problem.part_two("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
      "1"
    );
  }

  #[test]
  fn part_two_jump_imm_zero() {
    let problem = DayFive { inputs: vec![0] };
    assert_eq!(problem.part_two("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), "0");
  }

  #[test]
  fn part_two_jump_imm_nonzero() {
    let problem = DayFive { inputs: vec![-2] };
    assert_eq!(problem.part_two("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), "1");
  }
}
