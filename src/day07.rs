use std::ops::Range;

use itertools::Itertools;

use crate::intcode::{self, Intcode};
use crate::problem::Problem;

#[derive(Default)]
pub struct DaySeven {}

impl DaySeven {
  pub fn new() -> Self {
    Self {}
  }
}

const SERIES_PHASES: Range<u8> = (0..5);

impl Problem for DaySeven {
  fn soln_one(&self) -> String {
    "22012".to_string()
  }

  // 1st input: phase setting
  // 2nd input: amp's input signal (prev amp's output)
  fn part_one(&self, program: &str) -> String {
    let amp_program: Vec<i32> = intcode::parse_program(program);

    let mut all_outputs = Vec::new();

    let phase_permutations = SERIES_PHASES.permutations(SERIES_PHASES.len());
    for phase_settings in phase_permutations {
      let mut input_signal = 0;
      for &phase in phase_settings.iter() {
        let mut amp = amp_program.clone();
        let inputs = vec![phase as i32, input_signal];
        let outputs = Intcode::with_inputs(false, inputs).run(&mut amp);
        input_signal = outputs[0];
      }
      // output of amp sequence is the final signal
      let output_signal = input_signal;
      all_outputs.push(output_signal);
    }

    all_outputs.iter().max().map(i32::to_string).unwrap()
  }

}

#[cfg(test)]
mod tests {
  use super::DaySeven;
  use crate::problem::Problem;

  #[test]
  fn part_one_case1() {
    let problem = DaySeven::new();
    assert_eq!(
      problem.part_one("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
      "43210"
    );
  }

  #[test]
  fn part_one_case2() {
    let problem = DaySeven::new();
    assert_eq!(
      problem.part_one(
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
         101,5,23,23,1,24,23,23,4,23,99,0,0"
      ),
      "54321"
    );
  }

  #[test]
  fn part_one_case3() {
    let problem = DaySeven::new();
    assert_eq!(
      problem.part_one(
        "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
         1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
      ),
      "65210"
    );
  }

  #[test]
  fn part_two_equal_to_8_pos() {}
}
