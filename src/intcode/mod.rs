use std::convert::TryInto;
use std::fmt;
use std::io::{self, Write};

pub fn parse_program(program: &str) -> Vec<i32> {
  program
    .split(',')
    .map(|x| x.parse::<i32>())
    .filter_map(Result::ok)
    .collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Parameter {
  Position(u32),
  Immediate(i32),
}

impl fmt::Display for Parameter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Parameter::Position(p) => write!(f, "pos({})", p),
      Parameter::Immediate(v) => write!(f, "imm({})", v),
    }
  }
}

enum Instruction {
  Add(Parameter, Parameter, Parameter),
  Multiply(Parameter, Parameter, Parameter),
  Input(Parameter),
  Output(Parameter),
  JumpIfTrue(Parameter, Parameter),
  JumpIfFalse(Parameter, Parameter),
  LessThan(Parameter, Parameter, Parameter),
  Equal(Parameter, Parameter, Parameter),
  Halt,
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Instruction::Add(p1, p2, out) => write!(f, "ADD [{}, {}] -> {}", p1, p2, out),
      Instruction::Multiply(p1, p2, out) => write!(f, "MUL [{}, {}] -> {}", p1, p2, out),
      Instruction::Input(p) => write!(f, "INPUT -> {}", p),
      Instruction::Output(p) => write!(f, "OUTPUT [{}]", p),
      Instruction::JumpIfTrue(p, v) => write!(f, "JIT [{}] -> {}", p, v),
      Instruction::JumpIfFalse(p, v) => write!(f, "JIF [{}] -> {}", p, v),
      Instruction::LessThan(p1, p2, out) => write!(f, "LT [{}, {}] -> {}", p1, p2, out),
      Instruction::Equal(p1, p2, out) => write!(f, "EQ [{}, {}] -> {}", p1, p2, out),
      Instruction::Halt => write!(f, "HALT"),
    }
  }
}

pub struct Intcode {
  debug: bool,
  inputs: Vec<i32>,
}

const MAX_ITERS: u32 = 1_000_000;

impl Intcode {
  pub fn new(debug: bool) -> Self {
    Intcode {
      debug,
      inputs: Vec::new(),
    }
  }

  pub fn with_inputs(debug: bool, inputs: Vec<i32>) -> Self {
    Intcode { debug, inputs }
  }

  pub fn run(&self, program: &mut Vec<i32>) -> Vec<i32> {
    let mut outputs: Vec<i32> = Vec::new();

    let mut curr_input = 0;
    let mut ipr = 0;
    let mut iters = 0;

    while ipr < program.len() {
      iters += 1;
      if iters > MAX_ITERS {
        panic!("Ran too many times!");
      }

      let instruction = get_instruction(&program[ipr..]);
      if self.debug {
        println!("{}", instruction);
      }
      match instruction {
        Instruction::Add(p1, p2, out) => {
          match out {
            Parameter::Position(pos) => {
              let val1 = get_value(p1, &program);
              let val2 = get_value(p2, &program);
              program[pos as usize] = val1 + val2;
            }
            _ => panic!("Add output param must always be position!"),
          };

          ipr += 4;
        }
        Instruction::Multiply(p1, p2, out) => {
          match out {
            Parameter::Position(pos) => {
              let val1 = get_value(p1, &program);
              let val2 = get_value(p2, &program);
              program[pos as usize] = val1 * val2;
            }
            _ => panic!("Multiply output param must always be position!"),
          };

          ipr += 4;
        }
        Instruction::Input(loc) => {
          let inp: i32 = match self.inputs.get(curr_input) {
            Some(val) => *val,
            None => get_input(),
          };
          curr_input += 1;
          match loc {
            Parameter::Position(pos) => {
              program[pos as usize] = inp;
            }
            _ => panic!("Input param must always be position!"),
          };

          ipr += 2;
        }
        Instruction::Output(p) => {
          let output: i32 = match p {
            Parameter::Position(pos) => program[pos as usize],
            Parameter::Immediate(val) => val,
          };
          outputs.push(output);
          if self.debug {
            println!("Output: {}", output);
          }

          ipr += 2;
        }
        Instruction::JumpIfTrue(param, value) => {
          let should_jump = get_value(param, &program) != 0;
          if should_jump {
            ipr = match value {
              Parameter::Position(pos) => program[pos as usize] as usize,
              Parameter::Immediate(val) => val as usize,
            }
          } else {
            ipr += 3;
          }
        }
        Instruction::JumpIfFalse(param, value) => {
          let should_jump = get_value(param, &program) == 0;
          if should_jump {
            ipr = match value {
              Parameter::Position(pos) => program[pos as usize] as usize,
              Parameter::Immediate(val) => val as usize,
            }
          } else {
            ipr += 3;
          }
        }
        Instruction::LessThan(p1, p2, out) => {
          match out {
            Parameter::Position(pos) => {
              let val1 = get_value(p1, &program);
              let val2 = get_value(p2, &program);
              if val1 < val2 {
                program[pos as usize] = 1;
              } else {
                program[pos as usize] = 0;
              }
            }
            _ => panic!("LessThan output param must always be position!"),
          };

          ipr += 4;
        }
        Instruction::Equal(p1, p2, out) => {
          match out {
            Parameter::Position(pos) => {
              let val1 = get_value(p1, &program);
              let val2 = get_value(p2, &program);
              if val1 == val2 {
                program[pos as usize] = 1;
              } else {
                program[pos as usize] = 0;
              }
            }
            _ => panic!("LessThan output param must always be position!"),
          };

          ipr += 4;
        }
        Instruction::Halt => {
          return outputs;
        }
      };
    }

    panic!("Program finished without halting!");
  }
}

fn get_value(parameter: Parameter, program: &[i32]) -> i32 {
  match parameter {
    Parameter::Position(p) => program[p as usize],
    Parameter::Immediate(v) => v,
  }
}

fn get_instruction(program: &[i32]) -> Instruction {
  let opcode = program[0] as u32;
  match opcode % 100 {
    1 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      let param_2 = get_param(get_mode(opcode, 2), program[2]);
      let param_3 = get_param(get_mode(opcode, 3), program[3]);
      Instruction::Add(param_1, param_2, param_3)
    }
    2 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      let param_2 = get_param(get_mode(opcode, 2), program[2]);
      let param_3 = get_param(get_mode(opcode, 3), program[3]);
      Instruction::Multiply(param_1, param_2, param_3)
    }
    3 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      Instruction::Input(param_1)
    }
    4 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      Instruction::Output(param_1)
    }
    5 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      let param_2 = get_param(get_mode(opcode, 2), program[2]);
      Instruction::JumpIfTrue(param_1, param_2)
    }
    6 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      let param_2 = get_param(get_mode(opcode, 2), program[2]);
      Instruction::JumpIfFalse(param_1, param_2)
    }
    7 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      let param_2 = get_param(get_mode(opcode, 2), program[2]);
      let param_3 = get_param(get_mode(opcode, 3), program[3]);
      Instruction::LessThan(param_1, param_2, param_3)
    }
    8 => {
      let param_1 = get_param(get_mode(opcode, 1), program[1]);
      let param_2 = get_param(get_mode(opcode, 2), program[2]);
      let param_3 = get_param(get_mode(opcode, 3), program[3]);
      Instruction::Equal(param_1, param_2, param_3)
    }
    99 => Instruction::Halt,
    _ => panic!("Found unknown opcode {}", opcode),
  }
}

fn get_mode(opcode: u32, param_number: usize) -> u8 {
  assert!(param_number > 0, "param_number should be > 0");
  let divider = 10 * (10u32.pow(param_number as u32));
  ((opcode / divider) % 10).try_into().unwrap()
}

fn get_param(mode: u8, value: i32) -> Parameter {
  match mode {
    0 => Parameter::Position(value as u32),
    1 => Parameter::Immediate(value),
    _ => panic!("Found unknown parameter mode {} with value {}", mode, value),
  }
}

fn get_input() -> i32 {
  print!("Input: ");
  io::stdout().flush().unwrap();

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {}
    Err(error) => panic!("error: {}", error),
  };
  input.trim().parse().unwrap()
}

#[cfg(test)]
mod tests {
  use super::get_mode;

  #[test]
  fn get_mode_1() {
    assert_eq!(get_mode(1, 1), 0);
  }

  #[test]
  fn get_mode_2() {
    assert_eq!(get_mode(1101, 1), 1);
    assert_eq!(get_mode(1101, 2), 1);
    assert_eq!(get_mode(1101, 3), 0);
  }
}
