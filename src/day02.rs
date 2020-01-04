use crate::intcode::{self, Intcode};
use crate::problem::Problem;

#[derive(Default)]
pub struct DayTwo {}

impl DayTwo {
  pub fn new() -> Self {
    Self {}
  }
}

impl Problem for DayTwo {
  fn soln_one(&self) -> Option<String> {
    Some("2692315".to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let mut program: Vec<i64> = intcode::parse_program(input);

    // before running the program,
    // replace position 1 with the value 12 and
    // replace position 2 with the value 2.
    program[1] = 12;
    program[2] = 2;

    let mut intcode = Intcode::new(program);
    intcode.run();

    Some(intcode.memory.get_panic(0).to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some("9507".to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let base_program: Vec<i64> = intcode::parse_program(input);
    const TARGET: i64 = 19_690_720;

    for noun in 0..99 {
      for verb in 0..99 {
        let mut program = base_program.clone();
        program[1] = noun;
        program[2] = verb;
        let mut intcode = Intcode::new(program);
        intcode.run();
        if TARGET == *intcode.memory.get_panic(0) {
          return Some((noun * 100 + verb).to_string());
        }
      }
    }
    Some("Not found!".to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::intcode::Intcode;

  #[test]
  fn case1() {
    let mut intcode = Intcode::new(vec![1, 0, 0, 0, 99]);
    intcode.run();
    assert_eq!(intcode.memory.program, vec![2, 0, 0, 0, 99]);
  }

  #[test]
  fn case2() {
    let mut intcode = Intcode::new(vec![2, 3, 0, 3, 99]);
    intcode.run();
    assert_eq!(intcode.memory.program, vec![2, 3, 0, 6, 99]);
  }

  #[test]
  fn case3() {
    let mut intcode = Intcode::new(vec![2, 4, 4, 5, 99, 0]);
    intcode.run();
    assert_eq!(intcode.memory.program, vec![2, 4, 4, 5, 99, 9801]);
  }

  #[test]
  fn case4() {
    let mut intcode = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
    intcode.run();
    assert_eq!(intcode.memory.program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }
}
