/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.
unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let emojis = "🗻∈🌏";

    // Safe because the indices are in the correct order, within the bounds of
    // the string slice, and lie on UTF-8 sequence boundaries.
    unsafe {
        println!("emoji: {}", emojis.get_unchecked(0..4));
        println!("emoji: {}", emojis.get_unchecked(4..7));
        println!("emoji: {}", emojis.get_unchecked(7..11));
    }

    println!(
        "char count: {}",
        count_chars(unsafe { emojis.get_unchecked(0..7) })
    );

    // Not upholding the UTF-8 encoding requirement breaks memory safety!
    // println!("emojis: {}", unsafe { emojis.get_unchecked(0..3) });
    // println!(
    //     "char count: {}",
    //     count_chars(unsafe { emojis.get_unchecked(0..3) })
    // );

    let mut a = 42;
    let mut b = 66;

    unsafe {
        swap(&mut a, &mut b);
    }
    println!("a = {}, b = {}", a, b);

    unsafe {
        println!("Absolute value of -3 according to c: {}", abs(-3));
    }
}

fn count_chars(s: &str) -> usize {
    s.chars().map(|_| 1).sum()
}
