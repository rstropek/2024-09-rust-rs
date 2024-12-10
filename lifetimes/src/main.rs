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
}
