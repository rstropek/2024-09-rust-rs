fn main() {
    let numbers3;

    {
        let numbers = vec![1, 2, 3, 4, 5];
        println!("{}", numbers.len());
        
        let numbers2 = numbers;
        println!("{}", numbers2.len());
        
        numbers3 = numbers2;
    }
    
    println!("{}", numbers3.len());
    
    let mut numbers = create_vec();
    add_number(&mut numbers);
    print_numbers_better(&numbers);
    print_numbers(numbers);

    let numbers = vec![1, 2, 3, 4, 5];
    drop(numbers);

    let mut numbers = vec![1, 2, 3, 4, 5];
    let numbers2 = &numbers;
    let numbers3 = &numbers;
    println!("{}", numbers2.len());
    println!("{}", numbers3.len());
    
    let numbers2 = &mut numbers;
    numbers2.push(6);
    println!("{}", numbers2.len());

    let mut numbers = vec![1, 2, 3, 4, 5];
    let ro_borrowed_numbers = &numbers; // Read-only borrow of numbers
    // The following line is not possible because numbers is already borrowed immutably
    //let rw_borrowed_numbers = &mut numbers; // Read-write borrow of numbers
    println!("{}", ro_borrowed_numbers.len());
}

fn create_vec() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

fn print_numbers(numbers: Vec<i32>) {
    for number in numbers {
        println!("{}", number);
    }
}

fn print_numbers_better(numbers: &Vec<i32>) {
    for number in numbers {
        println!("{}", number);
    }
}

fn add_number(numbers: &mut Vec<i32>) {
    numbers.push(6);
}