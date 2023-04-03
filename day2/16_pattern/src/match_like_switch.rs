use std::io;

fn main() -> io::Result<()>{
    //let input = 'x';
    let mut input = String::new();

    println!("Please input a character: ");
    io::stdin().read_line(&mut input)?;

    let mut input_char = 'x';
    let count = input.char_indices().count();
    println!("{}", count);
    if count == 3 {
        input_char = input.chars().next().unwrap();
    }
    


    match input_char {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        _ => println!("Something else"),
    }

    Ok(())
}
