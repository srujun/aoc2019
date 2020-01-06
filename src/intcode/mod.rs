use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

pub fn parse_program(program: &str) -> Vec<i64> {
  program
    .split(',')
    .map(|x| x.parse::<i64>())
    .filter_map(Result::ok)
    .collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Parameter {
  Position(u64),
  Immediate(i64),
  Relative(i64),
}

impl Parameter {
  fn create(mode: u8, value: i64) -> Parameter {
    match mode {
      0 => Parameter::Position(value as u64),
      1 => Parameter::Immediate(value),
      2 => Parameter::Relative(value),
      _ => panic!("Found unknown parameter mode {} with value {}", mode, value),
    }
  }

  fn get_value(&self, memory: &Memory) -> i64 {
    match self {
      Parameter::Position(p) => *memory.get(*p as usize),
      Parameter::Immediate(v) => *v,
      Parameter::Relative(o) => *memory.get((memory.relative_base + *o) as usize),
    }
  }
}

impl fmt::Display for Parameter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Parameter::Position(p) => write!(f, "pos({})", p),
      Parameter::Immediate(v) => write!(f, "imm({})", v),
      Parameter::Relative(o) => write!(f, "rel({})", o),
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
  RelativeBaseOffset(Parameter),
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
      Instruction::RelativeBaseOffset(p) => write!(f, "RBO {}", p),
      Instruction::Halt => write!(f, "HALT"),
    }
  }
}

pub struct Memory {
  pub program: Vec<i64>,
  additional: HashMap<usize, i64>,
  relative_base: i64,
}

impl Memory {
  fn new(program: Vec<i64>) -> Self {
    Self {
      program,
      additional: HashMap::new(),
      relative_base: 0,
    }
  }

  fn exists(&self, address: usize) -> bool {
    address < self.program.len() || self.additional.contains_key(&address)
  }

  /// Gets the value at the specified `address`.
  /// Since Intcode is ok with non-existent addresses, it returns 0 in such cases.
  pub fn get(&self, address: usize) -> &i64 {
    self
      .program
      .get(address)
      .or_else(|| self.additional.get(&address))
      .unwrap_or(&0)
  }

  /// Gets the value at the address `relative_base + offset`.
  /// Since Intcode is ok with non-existent addresses, it returns 0 in such cases.
  pub fn get_rel(&self, offset: i64) -> &i64 {
    self.get((self.relative_base + offset) as usize)
  }

  fn set(&mut self, address: usize, value: i64) {
    if address < self.program.len() {
      self.program[address] = value;
    } else {
      self.additional.insert(address, value);
    }
  }

  fn set_rel(&mut self, offset: i64, value: i64) {
    self.set((self.relative_base + offset) as usize, value);
  }
}

pub struct Intcode {
  pub memory: Memory,
  pub debug: bool,
  pub inputs: Vec<i64>,
  next_input: usize,
  pub outputs: Vec<i64>,
  ipr: usize,
  iters: u32,
  pub has_halted: bool,
}

const MAX_ITERS: u32 = 1_000_000;

impl Intcode {
  pub fn new(program: Vec<i64>) -> Self {
    Intcode {
      memory: Memory::new(program),
      debug: false,
      inputs: Vec::new(),
      next_input: 0,
      outputs: Vec::new(),
      ipr: 0,
      iters: 0,
      has_halted: false,
    }
  }

  fn get_param(&self, opcode: u64, number: usize) -> Parameter {
    let value = *self.memory.get(self.ipr + number);
    Parameter::create(get_mode(opcode, number), value)
  }

  fn get_instruction(&self) -> Instruction {
    let opcode = *self.memory.get(self.ipr) as u64;
    match opcode % 100 {
      1 => {
        let param_1 = self.get_param(opcode, 1);
        let param_2 = self.get_param(opcode, 2);
        let param_3 = self.get_param(opcode, 3);
        Instruction::Add(param_1, param_2, param_3)
      }
      2 => {
        let param_1 = self.get_param(opcode, 1);
        let param_2 = self.get_param(opcode, 2);
        let param_3 = self.get_param(opcode, 3);
        Instruction::Multiply(param_1, param_2, param_3)
      }
      3 => {
        let param_1 = self.get_param(opcode, 1);
        Instruction::Input(param_1)
      }
      4 => {
        let param_1 = self.get_param(opcode, 1);
        Instruction::Output(param_1)
      }
      5 => {
        let param_1 = self.get_param(opcode, 1);
        let param_2 = self.get_param(opcode, 2);
        Instruction::JumpIfTrue(param_1, param_2)
      }
      6 => {
        let param_1 = self.get_param(opcode, 1);
        let param_2 = self.get_param(opcode, 2);
        Instruction::JumpIfFalse(param_1, param_2)
      }
      7 => {
        let param_1 = self.get_param(opcode, 1);
        let param_2 = self.get_param(opcode, 2);
        let param_3 = self.get_param(opcode, 3);
        Instruction::LessThan(param_1, param_2, param_3)
      }
      8 => {
        let param_1 = self.get_param(opcode, 1);
        let param_2 = self.get_param(opcode, 2);
        let param_3 = self.get_param(opcode, 3);
        Instruction::Equal(param_1, param_2, param_3)
      }
      9 => {
        let param_1 = self.get_param(opcode, 1);
        Instruction::RelativeBaseOffset(param_1)
      }
      99 => Instruction::Halt,
      _ => panic!("Found unknown opcode {}", opcode),
    }
  }

  pub fn run(&mut self) {
    if self.has_halted {
      panic!("Program has already halted!");
    }

    while self.memory.exists(self.ipr) {
      self.iters += 1;
      if self.iters > MAX_ITERS {
        panic!("Ran too many times!");
      }

      let instruction = self.get_instruction();
      if self.debug {
        println!("{}", instruction);
      }
      match instruction {
        Instruction::Add(p1, p2, out) => {
          let val1 = p1.get_value(&self.memory);
          let val2 = p2.get_value(&self.memory);
          match out {
            Parameter::Position(pos) => {
              self.memory.set(pos as usize, val1 + val2);
            }
            Parameter::Relative(off) => {
              self.memory.set_rel(off, val1 + val2);
            }
            _ => panic!("Add output param must always be position!"),
          };

          self.ipr += 4;
        }
        Instruction::Multiply(p1, p2, out) => {
          let val1 = p1.get_value(&self.memory);
          let val2 = p2.get_value(&self.memory);
          match out {
            Parameter::Position(pos) => {
              self.memory.set(pos as usize, val1 * val2);
            }
            Parameter::Relative(off) => {
              self.memory.set_rel(off, val1 * val2);
            }
            _ => panic!("Multiply output param must always be position!"),
          };

          self.ipr += 4;
        }
        Instruction::Input(loc) => {
          let inp: i64;
          match self.inputs.get(self.next_input) {
            Some(val) => {
              inp = *val;
              self.next_input += 1;
            }
            None => {
              if self.debug {
                println!("No input available, pausing execution...");
              }
              return;
            }
          };
          match loc {
            Parameter::Position(pos) => {
              self.memory.set(pos as usize, inp);
            }
            Parameter::Relative(off) => {
              self.memory.set_rel(off, inp);
            }
            _ => panic!("Input param must always be position!"),
          };

          self.ipr += 2;
        }
        Instruction::Output(p) => {
          let output = p.get_value(&self.memory);
          self.outputs.push(output);
          if self.debug {
            println!("Output: {}", output);
          }

          self.ipr += 2;
        }
        Instruction::JumpIfTrue(param, value) => {
          let should_jump = param.get_value(&self.memory) != 0;
          if should_jump {
            self.ipr = value.get_value(&self.memory) as usize;
          } else {
            self.ipr += 3;
          }
        }
        Instruction::JumpIfFalse(param, value) => {
          let should_jump = param.get_value(&self.memory) == 0;
          if should_jump {
            self.ipr = value.get_value(&self.memory) as usize;
          } else {
            self.ipr += 3;
          }
        }
        Instruction::LessThan(p1, p2, out) => {
          let val1 = p1.get_value(&self.memory);
          let val2 = p2.get_value(&self.memory);
          let output = if val1 < val2 { 1 } else { 0 };
          match out {
            Parameter::Position(pos) => {
              self.memory.set(pos as usize, output);
            }
            Parameter::Relative(off) => {
              self.memory.set_rel(off, output);
            }
            _ => panic!("LessThan output param must always be position!"),
          };

          self.ipr += 4;
        }
        Instruction::Equal(p1, p2, out) => {
          let val1 = p1.get_value(&self.memory);
          let val2 = p2.get_value(&self.memory);
          let output = if val1 == val2 { 1 } else { 0 };
          match out {
            Parameter::Position(pos) => {
              self.memory.set(pos as usize, output);
            }
            Parameter::Relative(off) => {
              self.memory.set_rel(off, output);
            }
            _ => panic!("LessThan output param must always be position!"),
          };

          self.ipr += 4;
        }
        Instruction::RelativeBaseOffset(p) => {
          let change: i64 = match p {
            Parameter::Position(pos) => *self.memory.get(pos as usize),
            Parameter::Immediate(val) => val,
            Parameter::Relative(off) => *self.memory.get_rel(off),
          };
          self.memory.relative_base += change;

          self.ipr += 2;
        }
        Instruction::Halt => {
          self.has_halted = true;
          return;
        }
      };
    }

    panic!("IPR={} points to null instruction!", self.ipr);
  }
}

fn get_mode(opcode: u64, param_number: usize) -> u8 {
  assert!(param_number > 0, "param_number should be > 0");
  let divider = 10 * (10u64.pow(param_number as u32));
  ((opcode / divider) % 10).try_into().unwrap()
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
