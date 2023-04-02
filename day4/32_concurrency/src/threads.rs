use std::{thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        for i in 0..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 0..5 {
        println!("Main thread: {i}!");
        thread::sleep(Duration::from_millis(5));
    }

    handler.join().unwrap();
}
