#![allow(unused_variables)]

fn main() {
    // Arrays
    let numbers = [1u32, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }

    let mut numbers = [0; 5];

    let number = numbers[5];
    numbers[5] = 10;

    // Tuples
    let point_2d = (1u32, 2);
    println!("x: {}, y: {}", point_2d.0, point_2d.1);

    let (x, y) = point_2d;
    println!("x: {}, y: {}", x, y);

    let points = ((1, 2), (3, 4));

    // Unit type
    let nothing = ();

    // Vectors
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    numbers.remove(0);
    println!("Length of numbers: {}", numbers.len());
    for n in numbers {
        println!("{}", n);
    }

    // Slices
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[1..3];
    println!("Slice: {:?}", slice);
    for n in slice {
        println!("{}", n);
    }
}
