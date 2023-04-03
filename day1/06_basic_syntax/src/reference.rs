fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    // let mut ref_x: &i32 = &mut x;  // a mutable reference which can be bound to different values
    *ref_x = 20;
    println!("x: {x}");
}