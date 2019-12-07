use std::env;
use std::fs;

use aoc2019::problem::Problem;

fn main() -> Result<(), String> {
  let args: Vec<String> = env::args().collect();

  match args.get(1) {
    Some(day) => {
      print_problem(day.parse::<usize>().expect("Invalid day input!"))?;
    }
    None => {
      for day in 1..25 {
        if let Err(_) = print_problem(day) {
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

  println!("Day {}", day_str);

  // Part 1
  let expected = problem.soln_one();
  let actual = problem.part_one(&input);
  println!("Part 1: (expected answer: {})", expected);
  println!("Actual: {} {}", actual, result(&expected, &actual));

  // Part 2
  let expected = problem.soln_two();
  let actual = problem.part_two(&input);
  println!("Part 2: (expected answer: {})", expected);
  println!("Actual: {} {}", actual, result(&expected, &actual));

  Ok(())
}

fn result(expected: &str, actual: &str) -> String {
  if expected == actual {
    "✓".to_string()
  } else {
    "✗".to_string()
  }
}

fn get_problem(day: usize) -> Option<Box<dyn Problem>> {
  match day {
    _ => None,
  }
}
