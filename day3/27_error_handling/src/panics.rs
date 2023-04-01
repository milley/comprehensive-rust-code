#[allow(unused)]
use std::panic;

fn main() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]); // panic

    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
}
