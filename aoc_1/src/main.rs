use std::fs::read_to_string;

fn main() {
    let content = read_to_string("input.txt").unwrap();
    println!("{}", content);
}
