mod helpers;
mod day1;

fn main() {
    println!("Advent of Code 2021!");
    println!("--------------------");

    let lines = helpers::read_file::lines_from_file("src/day1/day-1-input.txt");

    println!("d1 e1: {}", day1::exercise1(&lines));
    println!("d1 e2: {}", day1::exercise2(&lines));
}
