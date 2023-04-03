#[derive(Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.0
    }
}

#[allow(dead_code, non_camel_case_types)]
enum Option_i32 {
    Some(i32),
    None,
}

#[allow(dead_code, non_camel_case_types)]
enum Option_f64 {
    Some(f64),
    None,
}

#[allow(unused_variables)]
fn main() {
    let p = Point(5, 10);
    println!("p.x = {}", p.x());

    let p1 = Point(5.0, 10.0);
    println!("p1.x = {}", p1.x());


    let integer = Some(5);
    let float = Some(5.0);
    // The same as:
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}