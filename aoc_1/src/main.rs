use std::fs::read_to_string;

fn main() {
    let mut content = read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect::<(Vec<_>, Vec<_>)>();
    content.0.sort_unstable();
    content.1.sort_unstable();
    let sum = content.0.into_iter()
        .zip(content.1.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("{}", sum);

    //let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //let double_numbers: Vec<_> = (&numbers).into_iter().map(|x| x * 2).collect();
}


