use rand::Rng;

enum Result {
    Win,
    Draw,
    Lose,
}

fn main() {
    let rand_num = rand::thread_rng().gen_range(1..=10);

    let message = if rand_num > 5 {
        "Win"
    } else if rand_num == 5 {
        "Draw"
    } else {
        "Lose"
    };

    println!("{}", message);

    match rand_num {
        5 => println!("Draw"),
        n if n < 5 => println!("Lose"),
        _ => println!("Win"),
    }

    let message = match rand_num {
        5 => "Draw",
        n if n < 5 => "Lose",
        _ => "Win",
    };

    println!("{}", message);

    let result = match rand_num {
        5 => Result::Draw,
        i32::MIN..5 => Result::Lose,
        6..=i32::MAX => Result::Win,
    };

    let message = match result {
        Result::Win => "Win",
        Result::Draw => "Draw",
        Result::Lose => "Lose",
    };

    println!("{}", message);
}
