fn split_text<'a>(text: &'a str) -> (&'a str, u32) {
  let parts: Vec<&str> = text.split(" ").collect();

  return (parts[0], parts[1].parse::<u32>().unwrap());
}

pub fn exercise1(inputs: &Vec<String>) -> u32 {
  let mut forward = 0;
  let mut depth = 0;

  for input in inputs {
    match split_text(input.as_str()) {
      ("forward", val) => forward += val,
      ("down", val) => depth += val,
      ("up", val) => depth -= val,
      _ => continue,
    }
  }

  return forward * depth;
}

fn do_forward(forward: &mut u32, depth: &mut u32, aim: u32, val: u32) {
  *forward += val;
  *depth += val * aim; 
}

pub fn exercise2(inputs: &Vec<String>) -> u32 {
  let mut forward = 0;
  let mut depth = 0;
  let mut aim = 0;

  for input in inputs {
    match split_text(input.as_str()) {
      ("forward", val) => do_forward(&mut forward, &mut depth, aim, val),
      ("down", val) => aim += val,
      ("up", val) => aim -= val,
      _ => continue,
    }
  }

  return forward * depth;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_split_text() {
    assert_eq!(split_text("forward 5"), ("forward", 5));
  }

  #[test]
  fn test_exercise1() {
    assert_eq!(
      exercise1(&vec!(
        String::from("forward 5"),
        String::from("down 5"),
        String::from("forward 8"),
        String::from("up 3"),
        String::from("down 8"),
        String::from("forward 2"),
      )),
      150
    );
  }

  #[test]
  fn test_do_forward() {
    let mut forward = 0;
    let mut depth = 0;

    do_forward(&mut forward, &mut depth, 10, 8);

    assert_eq!(forward, 8);
    assert_eq!(depth, 80);
  }

  #[test]
  fn test_exercise2() {
    assert_eq!(
      exercise2(&vec!(
        String::from("forward 5"),
        String::from("down 5"),
        String::from("forward 8"),
        String::from("up 3"),
        String::from("down 8"),
        String::from("forward 2"),
      )),
      900
    );
  }
}
