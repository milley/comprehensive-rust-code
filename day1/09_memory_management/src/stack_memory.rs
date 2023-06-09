fn main() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");

    // DON'T DO THIS AT HOME! FOR educational purpose only.
    // String provide no guarantees about its layout, so this could lead to
    // undefined behavior.
    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }
}