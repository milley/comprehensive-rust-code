#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);


#[derive(Debug, Clone)]
struct PointStr(String, String);

fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");


    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    let ps1 = PointStr(String::from("3"), String::from("4"));
    let ps2 = ps1.clone();
    println!("ps1: {ps1:?}");
    println!("ps2: {ps2:?}");
}