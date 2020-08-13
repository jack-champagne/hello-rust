fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; //f32

    // Math operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // characters, unicode!
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // compound types
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // Equivalent to let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let third = a[2];

    let out_of_bounds = a[4]; // a[10];  --> main will panick because of array index being out of bounds
    
}
