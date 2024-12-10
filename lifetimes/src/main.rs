use rand::Rng;

#[derive(Clone)]
struct Customer {
    id: u32,
    name: String,
}

struct Order<'a> {
    id: u32,
    customer: &'a Customer,
    product: String,
    quantity: u32,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: (f32, f32),
    end: (f32, f32),
}

impl Line {
    fn length(&self) -> f32 {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        (dx * dx + dy * dy).sqrt()
    }
}

fn get_longer<'a>(line1: &'a Line, line2: &'a Line) -> &'a Line {
    if line1.length() > line2.length() {
        line1
    } else {
        line2
    }
}

fn get_longer_2<'a, 'b>(l1: &'a Line, l2: &'a Line, l3: &'b Line, l4: &'b Line) -> (&'a Line, &'b Line) {
    let longer1 = get_longer(l1, l2);
    let longer2 = get_longer(l3, l4);
    (longer1, longer2)
}

fn get_longer_str(s1: &'static str, s2: &'static str) -> &'static str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    {
        let customer = Customer {
            id: 1,
            name: "John Doe".to_string(),
        };

        let order = Order {
            id: 1,
            customer: &customer,
            product: "Laptop".to_string(),
            quantity: 1,
        };

        let order2 = Order {
            id: 2,
            customer: &customer,
            product: "Mouse".to_string(),
            quantity: 2,
        };

        println!("Order: {}", order.customer.name);
        println!("Order2: {}", order2.customer.name);

        drop(customer);

        println!("Hello world!");
    }

    {
        let customer = Customer {
            id: 1,
            name: "John Doe".to_string(),
        };
        let mut customers = vec![customer];

        let index = rand::thread_rng().gen_range(0..customers.len());

        let order = Order {
            id: 1,
            customer: &customers[index],
            product: "Laptop".to_string(),
            quantity: 1,
        };

        println!("Order: {}", order.customer.name);

        customers.remove(0);
    }

    let longer;
    let line1 = Line {
        start: (0.0, 0.0),
        end: (1.0, 1.0),
    };
    {
        let line2 = Line {
            start: (0.0, 0.0),
            end: (2.0, 2.0),
        };
        longer = get_longer(&line1, &line2);
        println!("Longer line: {:?}", longer);
    }

    println!("Line1: {:?}", line1);

    let s1 = "Hello";
    let s2 = "World";
    let longer = get_longer_str(s1, s2);
    println!("Longer string: {}", longer);
}
