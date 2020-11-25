// Introducing, if let to Rust! Its like match, but we only care about one of the conditions!

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three!"),
        _ => (),
    }

    // Instead of all the code to catch all the other conditions (in this case, the none)
    // We have a better syntax for this.
    if let Some(3) = some_u8_value {
        println!("Three")
    }

    // This is completely identical to the match expression above. The syntax takes in a pattern
    // and an expression separated by an equals sign.
    // We can also use else to execute code that is applicable for everything that does not
    // match the given pattern

    if let Some(4) = some_u8_value {
        println!("Four!")
    } else {
        println!("Not four!")
    }

    // This is a silly use case, but to be clear, if match seems a little too verbose for a problem
    // if let can also be your friend.
}
