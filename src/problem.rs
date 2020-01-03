pub trait Problem {
  fn soln_one(&self) -> Option<String> {
    None
  }

  fn part_one(&self, _input: &str) -> Option<String> {
    None
  }

  fn soln_two(&self) -> Option<String> {
    None
  }

  fn part_two(&self, _input: &str) -> Option<String> {
    None
  }
}
