// fn duplicate<T: Clone>(a: T) -> (T, T) {
//     (a.clone(), a.clone())
// }
fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

// syntatic sugar for:
//   fn add_42_millons<T: Into<i32>>(x: T) -> i32 {}
fn add_42_millons(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let many = add_42_millons(42_i8);
    println!("{many}");
    let many_more = add_42_millons(10_000_000);
    println!("{many_more}");

    let x = get_x("foo");
    println!("{x}");
}
