#[allow(dead_code)]
#[derive(Debug)]
struct Point<T, S> {
    x: T,
    y: S,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let different = Point { x: 5, y: 10.0 };
    println!("{integer:?}, {float:?}, {different:?}");
}
