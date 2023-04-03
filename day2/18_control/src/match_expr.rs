fn main() {
    // println!("{:?}", std::env::args());
    match std::env::args().next().as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls") => println!("Will ls some files"),
        Some("mv") => println!("Let's move some files"),
        Some("rm") => println!("Uh, dangerous!"),
        None => println!("Hmm, no program name?"),
        _ => println!("Unknown program name!"),
    }


    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}