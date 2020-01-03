use std::env;
use std::fs;

use colored::*;

use aoc2019::day01::DayOne;
use aoc2019::day02::DayTwo;
use aoc2019::day03::DayThree;
use aoc2019::day04::DayFour;
use aoc2019::day05::DayFive;
use aoc2019::day06::DaySix;
use aoc2019::day07::DaySeven;
use aoc2019::problem::Problem;

fn main() -> Result<(), String> {
  let args: Vec<String> = env::args().collect();

  match args.get(1) {
    Some(day) => {
      print_problem(day.parse::<usize>().expect("Invalid day input!"))?;
    }
    None => {
      for day in 1..25 {
        if print_problem(day).is_err() {
          break;
        }
        println!();
      }
    }
  }

  Ok(())
}

fn print_problem(day: usize) -> Result<(), String> {
  let day_str = format!("{:02}", day);

  let problem = get_problem(day).ok_or_else(|| format!("Day {} not implemented!", day_str))?;
  let input = fs::read_to_string(format!("inputs/day{}.txt", day_str)).unwrap();

  println!("{}", format!("DAY {}", day_str).blue().bold());
  print_part(1, &problem.soln_one(), &problem.part_one(&input));
  print_part(2, &problem.soln_two(), &problem.part_two(&input));

  Ok(())
}

fn print_part(num: usize, expected: &Option<String>, actual: &Option<String>) {
  println!(
    "Part {}: (expected answer: {})",
    num,
    expected.as_ref().unwrap_or(&"unknown".to_string()).bold()
  );
  println!(
    "Actual: {} {}",
    actual
      .as_ref()
      .unwrap_or(&"unimplemented".to_string())
      .bold(),
    result(&expected, &actual)
  );
}

fn result(expected: &Option<String>, actual: &Option<String>) -> String {
  if expected.is_none() {
    "??".yellow().to_string()
  } else if expected == actual {
    "✓".green().to_string()
  } else {
    "✗".red().to_string()
  }
}

fn get_problem(day: usize) -> Option<Box<dyn Problem>> {
  match day {
    1 => Some(Box::new(DayOne::new())),
    2 => Some(Box::new(DayTwo::new())),
    3 => Some(Box::new(DayThree::new())),
    4 => Some(Box::new(DayFour::new())),
    5 => Some(Box::new(DayFive::new())),
    6 => Some(Box::new(DaySix::new())),
    7 => Some(Box::new(DaySeven::new())),
    _ => None,
  }
}
