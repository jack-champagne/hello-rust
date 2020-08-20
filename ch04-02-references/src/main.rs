// This code is like the calculate_length code in ch04-01 but has been adapted to show how
// we can use references to simplify our code.

fn main() {
    let s1 = String::from("My heap string");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// This function takes a reference to a string as a parameter. Ownership is not passed 
// because there is no moving happening of values. Just creating a reference to the string

// &s1 lets us create a reference that *refers* to the value of s1 but does not own it.
// Because it does not own it, the value it points to will not be dropped when it goes
// out of scope.

// When passing references as function parameters, this is called borrowing with rust
// We cannot modify a borrowed value. The following code does not work without the mutable
// keyword

fn change(s: &mut String) {
    s.push_str(", and the modified stuff!");
}

// Example: 
// fn change(s: &String) {
//     s.push_str(", and the modified stuff!");
// }

// There is a restriction, you can only have one mutable reference to a specific piece of data in a single scope.
// The following will fail if line 44 is uncommented
fn too_many_references() {
    let mut s = String::from("Hello");

    let r1 = &mut s;
    //let r2 = &mut s;      // bad line of code

    println!("{}", r1);
}

fn smart_with_scope() {
    let mut s = String::from("This is going to have multiple, not concurrent references.");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}

fn immutable_and_mutable() {
    let mut s = String::from("Mutable and immutable");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;    // Code turns bad if line is: let r3 = &mut s;

    println!("{}, {}, {}", r1, r2, r3);
}

fn scope_affecting_references() {
    let mut s = String::from("This is my test string");

    let r1 = &s; // no problemo
    let r2 = &s; // also no problemo
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer being used any more and because of such, they are no longer in scope.

    let r3 = &mut s;
    println!("{}", r3);
}


// fn dangle() -> &String {
//     let s = String::from("My dangling string");
//     &s
// }   // owner of s is no longer in scope, reference to s is returned but s is freed from memory. S points to empty memory

//no dangle
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
// Ownership is passed in this example, nothing is freed in memory.

