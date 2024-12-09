#[derive(Clone, Eq, PartialEq, Debug)]
struct WallClock {
    hours: u8,
    minutes: u8,
}

impl WallClock {
    fn new(hours: u8, minutes: u8) -> Self {
        Self { hours, minutes }
    }

    fn add_minutes(&mut self, minutes: u8) {
        let total_minutes = self.minutes as u16 + minutes as u16;  // Convert to u16 to prevent overflow
        let additional_hours = total_minutes / 60;
        self.minutes = (total_minutes % 60) as u8;
        self.hours = ((self.hours as u16 + additional_hours) % 24) as u8;
    }

    fn add_minutes_2(&self, minutes: u8) -> Self {
        let total_minutes = self.minutes as u16 + minutes as u16;
        let additional_hours = total_minutes / 60;
        Self {
            hours: ((self.hours as u16 + additional_hours) % 24) as u8,
            minutes: (total_minutes % 60) as u8,
        }
    }
}

#[derive(Clone)]
struct Point2d {
    x: f32,
    y: f32,
}

fn main() {
    let mut clock = WallClock::new(12, 30);
    println!("The time is {:?}", clock);
    
    let borrowed_clock = &clock;
    let borrowed_clock = borrowed_clock.add_minutes_2(120);
    println!("The time is {} {}", borrowed_clock.hours, borrowed_clock.minutes);
    
    let mut_borrowed_clock = &mut clock;
    mut_borrowed_clock.add_minutes(120);
    println!("The time is {} {}", clock.hours, clock.minutes);

    let clock1 = WallClock::new(12, 30);
    let clock2 = clock1.clone();
    if clock1 == clock2 {
        println!("The clocks are equal");
    } else {
        println!("The clocks are not equal");
    }

    let point1 = Point2d { x: 1.0, y: 2.0 };
    let point2 = point1.clone();
    println!("The point is {} {}", point1.x, point1.y);
    println!("The point is {} {}", point2.x, point2.y);
    
}
