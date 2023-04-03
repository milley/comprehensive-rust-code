fn generate_random_number() -> i32 {
    let x = rand::random::<i32>();
    x
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        CoinFlip::Heads
    } else {
        CoinFlip::Tails
    }
}

fn main() {
    println!("You got: {:?}", flip_coin());
}