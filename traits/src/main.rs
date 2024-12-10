#![allow(dead_code)]

use std::fmt::Display;

trait Billable {
    fn total(&self) -> f32;
}

#[derive(Debug)]
struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl ConsultingWork {
    fn new(what: &str, hours: f32, rate: f32) -> Self {
        Self {
            what: what.to_string(),
            hours,
            rate,
        }
    }
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

fn print_total(b: &impl Billable) {
    println!("Total: {:.2}", b.total());
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

struct MyF32(f32); // Tuple struct ("newtype" pattern)

impl Display for MyF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The value is {:.2}", self.0)
    }
}

trait Pointworthy {
    fn points(&self) -> i32;
}

//impl<T> Pointworthy for T where T: Billable,
impl<T: Billable> Pointworthy for T 
{
    fn points(&self) -> i32 {
        (self.total() / 10.0) as i32
    }
}

fn print_points(p: &impl Pointworthy) {
    println!("Points: {}", p.points());
}

#[derive(Debug)]
enum ShirtSize {
    Small,
    Medium,
    Large
}

#[derive(Debug)]
struct CodingWork {
    size: ShirtSize,
}

impl Billable for CodingWork {
    fn total(&self) -> f32 {
        match self.size {
            ShirtSize::Small => 100.0,
            ShirtSize::Medium => 200.0,
            ShirtSize::Large => 300.0,
        }
    }
}

impl<T: Billable, const N: usize> Billable for [T; N] {
    fn total(&self) -> f32 {
        let mut sum = 0.0;
        for value in self {
            sum += value.total();
        }

        sum
    }
}

impl<T: Billable> Billable for Vec<T> {
    fn total(&self) -> f32 {
        let mut sum = 0.0;
        for value in self {
            sum += value.total();
        }

        sum
    }
}

fn main() {
    let work = ConsultingWork::new("Rust Training", 160.0, 150.0);
    println!("{:?}", work);
    print_total(&work);
    print_points(&work);
    let total = 1000.0;
    print_total(&total);
    print_points(&total);

    let my_f32 = MyF32(123.456);
    println!("{}", my_f32);

    let coding_work = CodingWork { size: ShirtSize::Large };
    println!("{:?}", coding_work);
    print_total(&coding_work);
    print_points(&coding_work);

    let numbers = [1.0, 2.0, 3.0, 5.0];
    println!("{:?}", numbers);
    print_total(&numbers);
    print_points(&numbers);

    let work_array = [
        ConsultingWork::new("Rust Training", 160.0, 150.0), 
        ConsultingWork::new("Wasm Training", 160.0, 150.0)
    ];
    println!("{:?}", work_array);
    print_total(&work_array);
    print_points(&work_array);
}

