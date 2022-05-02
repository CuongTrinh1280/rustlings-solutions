// functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
use std::convert::TryInto;

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));

    let integer_arch: usize = original_price.try_into().unwrap();
}

fn sale_price(price: i32) -> isize {
    if is_even(price) {
        (price - 10).try_into().unwrap()
    } else {
        (price - 3).try_into().unwrap()
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
