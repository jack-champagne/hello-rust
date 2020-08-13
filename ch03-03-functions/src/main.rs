fn main() {
    another_function(5);
    another_function1(5, 6);

    let y = 6;
    // let y = (let y = 6); This will give an error because this is a statement, not a expression

    // Statements and expressions
    let x = 5;

    // The curly brackets make this an expression, x + 1 evals to 4, y = 4
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// Cannot overload functions
fn another_function1(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1   // No semicolon because we want this function to return x + 1 as a expression
}