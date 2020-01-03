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
  fn soln_one(&self) -> String {
    "2692315".to_string()
  }

  fn part_one(&self, input: &str) -> String {
    let mut program: Vec<i32> = intcode::parse_program(input);

    // before running the program,
    // replace position 1 with the value 12 and
    // replace position 2 with the value 2.
    program[1] = 12;
    program[2] = 2;

    Intcode::new(false).run(&mut program);

    program[0].to_string()
  }

  fn soln_two(&self) -> String {
    "9507".to_string()
  }

  fn part_two(&self, input: &str) -> String {
    let base_program: Vec<i32> = intcode::parse_program(input);
    let target = 19_690_720;

    for noun in 0..99 {
      for verb in 0..99 {
        let mut program = base_program.clone();
        program[1] = noun;
        program[2] = verb;
        Intcode::new(false).run(&mut program);
        if target == program[0] {
          return (noun * 100 + verb).to_string();
        }
      }
    }
    "Not found!".to_string()
  }
}

#[cfg(test)]
mod tests {
  use crate::intcode::Intcode;

  #[test]
  fn case1() {
    let mut program = vec![1, 0, 0, 0, 99];
    Intcode::new(false).run(&mut program);
    assert_eq!(program, vec![2, 0, 0, 0, 99]);
  }

  #[test]
  fn case2() {
    let mut program = vec![2, 3, 0, 3, 99];
    Intcode::new(false).run(&mut program);
    assert_eq!(program, vec![2, 3, 0, 6, 99]);
  }

  #[test]
  fn case3() {
    let mut program = vec![2, 4, 4, 5, 99, 0];
    Intcode::new(false).run(&mut program);
    assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
  }

  #[test]
  fn case4() {
    let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    Intcode::new(false).run(&mut program);
    assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }
}
