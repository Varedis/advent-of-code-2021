pub fn exercise1(values: &[u32]) -> u32 {
  let mut count = 0;

  for i in 0..values.len() {
    if i == 0 { continue }

    let last = values[i - 1];

    if last < values[i] { count += 1 }
  }

  return count;
}

pub fn exercise2(values: &[u32]) -> u32 {
  let mut count = 0u32;

  for i in 0..values.len() {
    if i == 0 { continue }
    if i + 2 >= values.len() { break }

    let last_window_total = values[i-1] + values[i] + values[i+1];
    let next_window_total = values[i] + values[i+1] + values[i+2];

    if next_window_total > last_window_total { count += 1 }
  }
  
  return count;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exercise1() {
    assert_eq!(exercise1(&[1, 2, 3, 2, 95, 54]), 3);
  }

  #[test]
  fn test_exercise2() {
    assert_eq!(exercise2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5);
  }
}
