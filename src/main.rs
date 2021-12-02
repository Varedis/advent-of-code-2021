mod helpers;
mod day1;

fn main() {
    println!("Advent of Code 2021!");
    println!("--------------------");

    let input = helpers::format_file_lines_as_int("src/day1/input.txt");

    println!("d1 e1: {}", day1::exercise1(&input));
    println!("d1 e2: {}", day1::exercise2(&input));

    println!("d1 e1: {}", day1::exercise1(&lines));
    println!("d1 e2: {}", day1::exercise2(&lines));
}
