struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

#[allow(unused_variables)]
fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Existing block B");
        }
        println!("Existing block a");
    }
    drop(a);
    // a.drop();  // explicit destructor calls not allowed
    println!("Existing block main");
}
