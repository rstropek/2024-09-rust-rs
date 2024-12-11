use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

use rand::Rng;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let background1 = thread::spawn(move || {
        for i in 0..20 {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    let background2 = thread::spawn(move || {
        for i in 0..20 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    let rx = Arc::new(Mutex::new(rx));

    let mut consumers = Vec::new();
    for i in 0..3 {
        let rx = Arc::clone(&rx);
        let n = i;
        let consumer = thread::spawn(move || {
            loop {
                let received = rx.lock().unwrap().recv();
                match received {
                    Ok(num) => {
                        println!("Received: {} by {}", num, n);
                        // Simulate some work
                        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(10..100)));
                    }
                    Err(_) => break,
                }
            }
        });
        consumers.push(consumer);
    }

    background1.join().unwrap();
    background2.join().unwrap();

    for consumer in consumers {
        consumer.join().unwrap();
    }
}
