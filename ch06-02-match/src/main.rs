// This file will explore the match operator in rust. This operator allows
// different section of code to be run depending on the variant of a specified
// enumerator.

// The match operator, in practice, has a lot more functionality than listed below
// here but that will be out of scope of this chapter specifically. To detail 
// briefly, the match operator can compare a value against a series of patters
// and then execute the appropriate/corresponding code.

// Match is lke a coind sorting machine, the first hole that the coin will fit
// into, is the hole it falls down. Or quite literally as so:

#[derive(Debug)]
enum UsState {
    Alabama,
    Massachusetts,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {

    // Here is the syntax of a basic match expression. Each match expression will
    // have a comparison value, in this case, 'coin', and will also have 'arms'
    // that define behaviour per a specific match. The arms contain a pattern and
    // code to run separated by a => operator.

    // The matching of the value to a pattern of the arms happens in order to how
    // the arms are specified.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
    // For multi-line expression in a match arm, we can use curly brackets around
    // our code as so above

    // If you look in the Quarter pattern, the extra data (the state) that a quarter
    // can have is *bound* to the value of state in the code. This can be used to get
    // the inner type 'UsState' out of the Coin::Quarter
}

// Here is an example of using Option enum to avoid all the problems that come with null!
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1),
    }
}

// Second to last concept, matches are Exhaustive
// Here is some example code that will not compile
fn plus_one_bad(x: Option<i32>) -> Option<i32> {
    match x {
        Some(val) => Some(val + 1)
    }
}

// Last important concept... the _ placeholder acts as a catch all for pattern mathching.
// Having to be exhaustive is great and all, but it is tough to list out all the possible
// scenarios all the time. The special pattern _ matches to all patters and can be used
// like so:
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_val = 0u8;
    match some_u8_val {
        1 => println!("One!"),
        3 => println!("Three!"),
        6 => println!("Six!"),
        _ => (),
    }
}

// This awesome power can be a little cumbersom if we only care about one of the cases. Onward!