use rand::Rng;

#[derive(PartialEq, Eq)]
struct Guest {
    name: String,
}

#[derive(PartialEq, Eq)]
struct MaintenanceWork {
    what: String,
    costs: i32,
}

#[derive(PartialEq, Eq)]
enum HotelRoom {
    Vacant,
    Occupied(Guest),
    Maintenance(MaintenanceWork),
}

impl HotelRoom {
    fn new_vacant() -> Self {
        HotelRoom::Vacant
    }

    fn new_occupied(guest: Guest) -> Self {
        HotelRoom::Occupied(guest)
    }

    fn new_maintenance(work: MaintenanceWork) -> Self {
        HotelRoom::Maintenance(work)
    }
}

fn main() {
    let room = HotelRoom::new_vacant();
    let room = HotelRoom::new_occupied(Guest { name: "John".to_string() });
    let room = HotelRoom::new_maintenance(MaintenanceWork { what: "Cleaning".to_string(), costs: 100 });
    
    match room {
        HotelRoom::Vacant => println!("Room is vacant"),
        HotelRoom::Occupied(guest) => println!("Room is occupied by {}", guest.name),
        HotelRoom::Maintenance(work) => println!("Room is under maintenance: {} for ${}", work.what, work.costs),
    }
    
    let room = HotelRoom::Maintenance(MaintenanceWork { what: "Cleaning".to_string(), costs: 100 });
    if room == HotelRoom::Vacant {
        println!("Room is vacant");
    }

    if let HotelRoom::Occupied(guest) = room {
        println!("Room is occupied by {}", guest.name);
    }

    let mut g = Some(Guest { name: "John".to_string() });
    g = None;

    let number = maybe_a_number();
    if let Some(n) = number {
        println!("The number is {}", n);
    }

    let res = div(10, 2);
    match res {
        Ok(n) => println!("The result is {}", n),
        Err(e) => println!("Error: {}", e),
    }

    if let Ok(n) = div(10, 2) {
        println!("The result is {}", n);
    }
}

fn maybe_a_number() -> Option<i32> {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0..100);
    if random_number % 2 == 0 {
        Some(random_number)
    } else {
        None
    }
}

fn div(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}