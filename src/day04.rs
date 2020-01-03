use crate::problem::Problem;

#[derive(Default)]
pub struct DayFour {}

impl DayFour {
  pub fn new() -> Self {
    Self {}
  }

  fn check_part_one(num: u32) -> bool {
    Self::non_decreasing(num) && Self::adjacent_same(num)
  }

  fn check_part_two(num: u32) -> bool {
    Self::non_decreasing(num) && Self::two_adjacent_same(num)
  }

  fn digits(num: u32) -> Vec<u32> {
    num
      .to_string()
      .chars()
      .map(|c| c.to_digit(10).unwrap())
      .collect()
  }

  fn non_decreasing(num: u32) -> bool {
    let mut prev = 0;
    for digit in Self::digits(num) {
      if digit < prev {
        return false;
      }
      prev = digit;
    }
    true
  }

  fn adjacent_same(num: u32) -> bool {
    let mut all = Self::digits(num).into_iter();
    let mut prev = all.next().unwrap();
    for digit in all {
      if digit == prev {
        return true;
      }
      prev = digit;
    }
    false
  }

  fn two_adjacent_same(num: u32) -> bool {
    let mut all = Self::digits(num).into_iter();
    let mut prev = all.next().unwrap();
    let mut count = 1;
    for digit in all {
      if digit == prev {
        count += 1;
      } else {
        if count == 2 {
          return true;
        }
        count = 1;
      }
      prev = digit;
    }
    // group is at the end
    if count == 2 {
      return true;
    }
    false
  }
}

impl Problem for DayFour {
  fn soln_one(&self) -> Option<String> {
    Some("1686".to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let mut nums = input.split('-');
    let start: u32 = nums.next().unwrap().parse().unwrap();
    let end: u32 = nums.next().unwrap().parse().unwrap();

    Some(
      (start..=end)
        .filter(|&num| Self::check_part_one(num))
        .count()
        .to_string(),
    )
  }

  fn soln_two(&self) -> Option<String> {
    Some("1145".to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let mut nums = input.split('-');
    let start: u32 = nums.next().unwrap().parse().unwrap();
    let end: u32 = nums.next().unwrap().parse().unwrap();

    Some(
      (start..=end)
        .filter(|&num| Self::check_part_two(num))
        .count()
        .to_string(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::DayFour;

  #[test]
  fn part_one_case1() {
    assert_eq!(true, DayFour::check_part_one(111_111));
  }

  #[test]
  fn part_one_case2() {
    assert_eq!(false, DayFour::check_part_one(223_450));
  }

  #[test]
  fn part_one_case3() {
    assert_eq!(false, DayFour::check_part_one(123_789));
  }

  #[test]
  fn part_two_case1() {
    assert_eq!(true, DayFour::check_part_two(112_233));
  }

  #[test]
  fn part_two_case2() {
    assert_eq!(false, DayFour::check_part_two(123_444));
  }

  #[test]
  fn part_two_case3() {
    assert_eq!(true, DayFour::check_part_two(111_122));
  }
}
