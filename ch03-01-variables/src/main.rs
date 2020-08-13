fn main() {
    // Immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Variable shadowing
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // Cannot be mutable because variable types cannot be mutated!
    let spaces = "   ";
    let spaces = spaces.len();
}
