static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("HELLO_WORLD: {HELLO_WORLD}");

    add_to_counter(42);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
