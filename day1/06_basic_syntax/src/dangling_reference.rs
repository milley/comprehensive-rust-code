fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 0;
        ref_x = &x;  // forbid dangling reference
    }

    println!("ref_x: {ref_x}");
}