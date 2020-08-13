fn main() {
    let num = 7;
    if num < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // Condition must be a bool!
    // if num {         // This will throw an error
    // let x = 5;
    // }

    let num = 4;

    if num != 0 {
        println!("num was something other than zero");
    }

    let num = 6;

    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2")
    }

    // If you have more than one else if (like this) match might be better for you

    // if let statement
    let condition = true;
    let number = if condition { 4 } else { 6 }; // These two execution path expressions must have the same type

    println!("The value of number is: {}", number);

    // loops
    loop {
        println!("again!");
        break;
    }

    
    // Expression from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!");

    // looping through array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // using a for loop for collection
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF AGAIN!");
}
