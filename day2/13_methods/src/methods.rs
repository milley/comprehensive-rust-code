#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}, i'm {} years old", self.name, self.age);
    }
}

fn main() {
    let peter = Person {
        name: String::from("peter"),
        age: 27,
    };
    peter.say_hello();
}
