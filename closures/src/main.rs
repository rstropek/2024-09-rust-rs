/*
trait MyAnswerer {
    fn get_answer(&self, y: u32) -> u32;
}

struct GetAnswerClosure {
    x: u32
}

impl MyAnswerer for GetAnswerClosure {
    fn get_answer(&self, y: u32) -> u32 {
        self.x + y
    }
}
*/

use std::{thread, time::Duration}; 

struct Something {
    s1: String,
    s2: String,
}

fn main() {
    let x = 21;
    let get_answer = |y| x + y;

    // {
    //     let helper = GetAnswerClosure { x: 21u32 };
    //     println!("{}", helper.get_answer(1));
    //     calc_and_print_answerer(1, helper);
    // }

    println!("{}", get_answer(1));
    println!("{}", (|y| x + y)(15));

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    calc_and_print(1, 2, Box::new(add));
    calc_and_print(1, 2, Box::new(|x, y| x - y));

    let z = 10;
    calc_and_print(5, 10, Box::new(|x, y| x + y + z));

    let z = 20;
    calc_and_print(5, 10, Box::new(|x, y| x + y * z));
    println!("{}", z);

    let mut result = 0;
    let mut calc_result = |x, y| { result = x + y; };
    calc_result(1, 2);
    println!("{}", result);

    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();
    let sum_calculator = || numbers_iter.sum::<i32>();
    println!("{}", sum_calculator());

    let mut some = Something { s1: "Hello".to_string(), s2: "World".to_string() };
    let background = thread::spawn(move || {
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(10));
            some.s1.push('*');
        }
        println!("{}", some.s1);
        println!("Background thread finished");
    });
    
    background.join().unwrap();
    println!("{}", some.s2);
    println!("Main thread finished");
}

fn calc_and_print(x: i32, y: i32, calc_strategy: Box<dyn Fn(i32, i32) -> i32 + '_>) {
    let result = calc_strategy(x, y);
    println!("{}", result);
}
