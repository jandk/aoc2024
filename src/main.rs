use std::fs::read_to_string;

mod day1;

fn main() {
    println!("{}", day1::part1(&read_to_string("day1.txt").unwrap()));
    println!("{}", day1::part2(&read_to_string("day1.txt").unwrap()));
}
