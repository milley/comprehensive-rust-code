fn say_hello(name: String) {
    println!("Hello {name}");
}

fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");   // moved


    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);    //moved
}