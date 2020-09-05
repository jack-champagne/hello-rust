fn main() {
    println!("Hello, world!");
    let test = String::from("Hello, world!");
    println!("{}", first_word(&test));

    println!("{}", better_first_word(&test[..]));
    println!("{}", better_first_word("Hello, world!"));
}

// Slices do not have ownership. They reference a continguous sequence of elements in
// a collection rather than the whole collection

// Programming problem proposed find first word in string and if there are no spaces,
// return the entire string.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}
// ^ This is a solution to the original problem, but programmitacally what if we were to
// do something like this:
fn example() {
    let mut s = String::from("Hello world!");

    let word = first_word(&s);
    s.clear();

    println!("{}", word)
}
// Here the function would be called, a value returned and that value would remain the
// same unless we called the function again. So, word = 5 after line 28, and when we clear
// the string, word = 5 still remains even though s has changed. What can we do to correct
// this? And what would happen if we started doing something like second_word()? now we
// have a ripe mess
// String slices to the rescue! They are references to part of a String.alloc
fn example2() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];  // Like making a reference to string except for the [0..5] bit
    
    let same_string = &s[0..2];
    let same_string = &s[..2];  // These are equal

    let whole_string = &s[..];  // This makes a slice of the entire string
}
// Slices are stored internally with a pointer to the start of the slice and a length value
// The declaration is [starting index..ending index] where the ending index is exclusive
// What if we return a slice instead for first_word?!
fn better_first_word(s : &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s // or &s[..], these are the same
}
// Now we can write a good second_word function
// fn second_word (s: &String) -> &str {}
// Also, a cool note, the compiler ensures that the references to string stay valid!
// There will be a compile time error is we tried to do this:
fn example3() {
    let mut s = String::from("Hello world! This is a long string.");
    let word = better_first_word(&mut s); // This line
    // s.clear();  // This is a mutable borrow, and the compiler does not like it because 
                //there is already an immutable reference or there is already a mutable
                // reference depending on line above.

    println!("{}", word)
}

// String literals are slices
// When we create string literals, they are compiled into the binary. The literal *actually*
// contains a slice (&str) that points to a specific part of the binary. This is also why string
// literals are also immutable. &str is an immutable reference.
// We can also write this
// fn first_word(s: &str) -> &str {} for our function signature. This will be the implementation above.
// There are more than just string slices. 
fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];   // This will have type &[i32], works exactly the same as string references internally.
}

// Next up, structs!