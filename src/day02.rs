use crate::problem::Problem;

pub struct DayTwo {}

impl DayTwo {
  fn solve(&self, program: &mut Vec<u32>) {
    let mut ipr = 0;
    let mut iters = 0;
    while ipr < program.len() {
      iters += 1;
      if iters > 1000 {
        panic!("Ran too many times!");
      }
      match program[ipr] {
        1 => {
          let a_loc = program[ipr + 1] as usize;
          let b_loc = program[ipr + 2] as usize;
          let r_loc = program[ipr + 3] as usize;
          program[r_loc] = program[a_loc] + program[b_loc];
        }
        2 => {
          let a_loc = program[ipr + 1] as usize;
          let b_loc = program[ipr + 2] as usize;
          let r_loc = program[ipr + 3] as usize;
          program[r_loc] = program[a_loc] * program[b_loc];
        }
        99 => break,
        _ => panic!("Found unknown opcode {} at pos {}", program[ipr], ipr),
      };
      ipr += 4;
    }
  }
}

impl Problem for DayTwo {
  fn soln_one(&self) -> String {
    "2692315".to_string()
  }

  fn part_one(&self, input: &str) -> String {
    let mut program: Vec<u32> = input
      .split(',')
      .map(|x| x.parse::<u32>())
      .filter_map(Result::ok)
      .collect();

    // before running the program,
    // replace position 1 with the value 12 and
    // replace position 2 with the value 2.
    program[1] = 12;
    program[2] = 2;

    self.solve(&mut program);

    program[0].to_string()
  }

  fn soln_two(&self) -> String {
    "9507".to_string()
  }

  fn part_two(&self, input: &str) -> String {
    let base_program: Vec<u32> = input
      .split(',')
      .map(|x| x.parse::<u32>().unwrap())
      .collect();
    let target = 19_690_720;

    for noun in 0..99 {
      for verb in 0..99 {
        let mut program = base_program.clone();
        program[1] = noun;
        program[2] = verb;
        self.solve(&mut program);
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
  use super::DayTwo;

  #[test]
  fn case1() {
    let problem = DayTwo {};
    let mut program = vec![1, 0, 0, 0, 99];
    problem.solve(&mut program);
    assert_eq!(program, vec![2, 0, 0, 0, 99]);
  }

  #[test]
  fn case2() {
    let problem = DayTwo {};
    let mut program = vec![2, 3, 0, 3, 99];
    problem.solve(&mut program);
    assert_eq!(program, vec![2, 3, 0, 6, 99]);
  }

  #[test]
  fn case3() {
    let problem = DayTwo {};
    let mut program = vec![2, 4, 4, 5, 99, 0];
    problem.solve(&mut program);
    assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
  }

  #[test]
  fn case4() {
    let problem = DayTwo {};
    let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    problem.solve(&mut program);
    assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
  }
}
