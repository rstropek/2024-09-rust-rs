#[derive(Debug)]
struct MyI32(i32);

struct CounterIterator {
    current: i32,
    end: i32,
}

impl Iterator for CounterIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

type FibonacciType = i32;

struct Fibonacci {
    current: FibonacciType,
    next: FibonacciType,
}

impl Iterator for Fibonacci {
    type Item = FibonacciType;

    fn next(&mut self) -> Option<Self::Item> {
        println!("next");
        let result = self.current;
        self.current = self.next;
        self.next = result + self.next;
        Some(result)
    }
}

fn main() {
    let mut numbers = vec![MyI32(4711),];
    //for n in numbers {
    //    println!("{n:?}");
    //}

    let mut iter = (numbers).into_iter();
    while let Some(n) = iter.next() {
        println!("{n:?}");
    }

    let ci = CounterIterator {
        current: 0,
        end: 10,
    };
    for n in ci {
        println!("{n}");
    }

    let fib = Fibonacci {
        current: 0,
        next: 1,
    };
    println!("Before");
    let my_fib = fib.skip(2).take(10);
    println!("After");
    for fib in my_fib {
        println!("{fib}");
    }
}
