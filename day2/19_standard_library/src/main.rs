use std::collections::HashMap;

fn option_test() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {:?}", first);

    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {:?}", idx);
}

fn string_test() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨 🇭");
    println!("s3: len = {}, capacity = {}", s3.len(), s3.capacity());
}

fn vec_test() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    v3.dedup();
    println!("{v3:?}");
}

fn hashmap_test() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We known about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");

    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone ")
        .unwrap_or(&336);
    println!("pc1: {pc1}");
    let pc2 = page_counts
        .entry("The Hunger Games".to_string())
        .or_insert(374);
    println!("pc2: {pc2}");

    let page_counts = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);
    println!("{page_counts:#?}");
}

fn main() {
    option_test();
    string_test();
    vec_test();
    hashmap_test();
}
