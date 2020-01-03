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

  fn get_output_part_one(phase_settings: &[u8], amp_program: &[i64]) -> i64 {
    let mut signal = 0;
    for &phase in phase_settings.iter() {
      let mut intcode = Intcode::new(amp_program.to_vec());
      intcode.inputs = vec![phase as i64, signal];
      intcode.run();
      signal = intcode.outputs[0];
    }
    // output of amp sequence is the final signal
    signal
  }

  fn get_output_part_two(phase_settings: &[u8], amp_program: &[i64]) -> i64 {
    // can probably be optimized...
    let mut amps: [Intcode; NUM_AMPS] = [
      Intcode::new(amp_program.to_vec()),
      Intcode::new(amp_program.to_vec()),
      Intcode::new(amp_program.to_vec()),
      Intcode::new(amp_program.to_vec()),
      Intcode::new(amp_program.to_vec()),
    ];
    // provide the initial phase signal to each amp
    for (i, amp) in amps.iter_mut().enumerate() {
      amp.inputs.push(phase_settings[i] as i64);
    }
    // provide 0 input signal to first amp
    amps[0].inputs.push(0);

    // cycle through each amp in order
    let mut amp_iter = (0..NUM_AMPS).cycle().peekable();
    loop {
      let signal: i64;
      // separate blocks to not mutably borrow amps more than
      // once at the same time
      {
        let curr_amp: &mut Intcode = &mut amps[amp_iter.next().unwrap()];
        curr_amp.run();
        signal = *curr_amp.outputs.last().unwrap();
      }
      {
        let next_amp: &mut Intcode = &mut amps[*(amp_iter.peek().unwrap())];
        if next_amp.has_halted {
          break;
        } else {
          next_amp.inputs.push(signal);
        }
      }
    }

    // output of amp sequence is the final signal
    *amps[amps.len() - 1].outputs.last().unwrap()
  }
}

const NUM_AMPS: usize = 5;
const SERIES_PHASES: Range<u8> = (0..5);
const LOOP_PHASES: Range<u8> = (5..10);

impl Problem for DaySeven {
  fn soln_one(&self) -> Option<String> {
    Some("22012".to_string())
  }

  // 1st input: phase setting
  // 2nd input: amp's input signal (prev amp's output)
  fn part_one(&self, program: &str) -> Option<String> {
    let amp_program: Vec<i64> = intcode::parse_program(program);

    let phase_permutations = SERIES_PHASES.permutations(NUM_AMPS);
    let all_outputs: Vec<i64> = phase_permutations
      .map(|setting| Self::get_output_part_one(&setting, &amp_program))
      .collect();

    // find the largest output
    Some(all_outputs.iter().max().map(i64::to_string).unwrap())
  }

  fn soln_two(&self) -> Option<String> {
    Some("4039164".to_string())
  }

  fn part_two(&self, program: &str) -> Option<String> {
    let amp_program: Vec<i64> = intcode::parse_program(program);

    let phase_permutations = LOOP_PHASES.permutations(NUM_AMPS);
    let all_outputs: Vec<i64> = phase_permutations
      .map(|setting| Self::get_output_part_two(&setting, &amp_program))
      .collect();

    // find the largest output
    Some(all_outputs.iter().max().map(i64::to_string).unwrap())
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
      problem
        .part_one("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
        .unwrap(),
      "43210"
    );
  }

  #[test]
  fn part_one_case2() {
    let problem = DaySeven::new();
    assert_eq!(
      problem
        .part_one(
          "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
           101,5,23,23,1,24,23,23,4,23,99,0,0"
        )
        .unwrap(),
      "54321"
    );
  }

  #[test]
  fn part_one_case3() {
    let problem = DaySeven::new();
    assert_eq!(
      problem
        .part_one(
          "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
           1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
        )
        .unwrap(),
      "65210"
    );
  }

  #[test]
  fn part_two_case1() {
    let problem = DaySeven::new();
    assert_eq!(
      problem
        .part_two(
          "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
           27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        )
        .unwrap(),
      "139629729"
    );
  }

  #[test]
  fn part_two_case2() {
    let problem = DaySeven::new();
    assert_eq!(
      problem
        .part_two(
          "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
           -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
           53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
        )
        .unwrap(),
      "18216"
    );
  }
}
