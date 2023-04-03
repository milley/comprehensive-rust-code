#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn create_default() {
    let tmp = Person {
        ..Default::default()
    };
    println!("{tmp:?}");

    let tmp1 = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
    println!("{tmp1:?}");
}

struct Point(i32, i32);


fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);

    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);

    let jackie = Person {
        name: String::from("Jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);

    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    create_default();

    let bob = Person::new(String::from("Bob"), 35);
    println!("{bob:?}");
}
