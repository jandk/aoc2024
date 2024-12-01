use std::collections::{HashMap, HashSet};
use std::iter::zip;

pub(crate) fn part1(input: &str) -> i32 {
    let (mut array1, mut array2) = parse(input);

    array1.sort();
    array2.sort();

    zip(array1, array2).map(|(a, b)| (b - a).abs()).sum()
}

pub(crate) fn part2(input: &str) -> i32 {
    let (array1, array2) = parse(input);

    let mut map: HashMap<i32, i32> = HashMap::new();
    array2.iter().for_each(|&x| {
        map.entry(x).and_modify(|x| *x += 1).or_insert(1);
    });

    array1.iter().map(|&x| x * *map.entry(x).or_default()).sum()
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let numbers = input
        .lines()
        .flat_map(|line| line.split_whitespace().map(|s| s.parse().unwrap()))
        .collect::<Vec<i32>>();

    (
        numbers.chunks_exact(2).map(|i| i[0]).collect::<Vec<_>>(),
        numbers.chunks_exact(2).map(|i| i[1]).collect::<Vec<_>>(),
    )
}
