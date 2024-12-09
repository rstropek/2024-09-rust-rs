use rand::{Rng, thread_rng};

fn main() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("The sum of {} and {} is {}", a, b, c);

    let a = 3;
    let numbers = [1, 2, 3, 4, 5];
    let b = numbers[a];
    println!("The value of b is {}", b);

    let number = "42"; // In reality, the user enters this number via console
    let number: i32 = number.parse().unwrap();
    println!("The value of number is {}", number);

    let mut a = 10;
    a += 1;
    println!("The value of a is {}", a);

    let a = a;

    let mut _a = a;
    _a += 1;

    let sum = add(1, 2);
    println!("The sum of 1 and 2 is {}", sum);

    let div = div(1, 2);
    println!("The division of 1 and 2 is {}", div);

    let num = thread_rng().gen_range(0..=100);
    println!("The random number is {}", num);
}

fn add(a: i32, b: i32) -> i32 {
    println!("Adding {} and {}", a, b);
    a + b
}

fn div(a: i32, b: i32) -> i32 {
    let result = if b == 0 { 0 } else { a / b };
    result
}
