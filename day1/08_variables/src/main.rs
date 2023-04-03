fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x: i32 = 10;
    println!("x: {x}");

    // x = 20;
    // println!("x: {x}");


    let x = 10;
    let y = 20;
    takes_u32(x);
    takes_i8(y);

    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
}
