// NOTES
// * Rust's ownership system provides a different way of managing memory than other 
//   languages (explicitly allocating and freeing memory or garbage collection)
// * Stack and heap allocation actually matter
// * Ownership rules:
//      1. Each value in Rust has a variable that's called its owner
//      2. There can only be one owner at a time.
//      3. When the owner goes out of scope, the value will be dropped (does this mean the memory is freed?)
// * The ownership of a variable follows the same pattern every time: assigning a value to another variable
//   moves it or copies it depending on trait. This cannot change.
// 

fn main() {

    // Scope
    {   // s is not yet valid because it has not been declared
        let s = "hi there"; // s is now valid
        // s is still valid
    }   // s is not valid, out of scope

    let mut s = "test";
    // s = "hello, " + "there";     - can't do this
    // s = s + " test 2";           - can't do this either
    s = "hello";                  //- can do this

    // All of the data types that were previously discussed (except for Strings) are allocated and popped
    // off the stack once they are out of scope. They are 'simple' data types (I call them primitives
    // because of my Java background)

    // To allocate a string of unknown size at compile time (let's say text from file or use input) we can
    // allocate on the heap
    let mut s = String::from("this is a heap string lol");
    // :: allows us to select functions from the String namespace (like c++)
    // This string can be mutated

    s.push_str(", more string that mutated original!");
    println!("{}", s);

    // String literals compile directly into final executable, that's why they're so fast and efficient.
    // We *cannot* allocate a string literal that can vary in size because of this. We must use the HEAP
    // To use the heap, we must first request memory from the memory allocator at runtime (is this an OS
    // thing?) and then we must release the memory back to the allocator when we are done.

    // Doing this is harder said than done. Some languages have a GC do this during runtime. Others you
    // must free manually at the right time (and the right number of times) to not cause an error or
    // memory leak. Here is Rust's path:
    {
        let s = String::from("hello heap!");    // s is declared
    }   // the scope is over and s is no longer valid, memory gets returned.
    // Rust calls a special functions called 'drop' automatically at the end of the curly brackets for
    // String. The author of String can write the code there to return the memory.

    // What happens when multiple variables interact with data on the heap?
    // Stack allocated literals
    let x = 5;
    let y = x;
    // Heap allocated data
    let s1 = String::from("This is on the heap, fun with data");
    let s2 = s1;

    // What's the difference here? In the literals example, y makes a copy of the value that x holds (5)
    // and then binds y to the value 5. Each variable contains a reference to a seperate (5) in memory
    // In the second part, s1 contains a pointer, a length, and a capacity for its string. When s1 is
    // assigned to s2, it makes a copy of its contents. It does NOT create a copy of the data that the
    // pointer points to. Both variables are pointing to the same data. This is efficient, not making a
    // copy of the data and just copying the reference is faster especially for large amounts of data.

    // If both s1 and s2 go out of scope, they will attempt to free the same memory, which is a problem.
    // Rust alleviates this by invalidating s1 when s2 copies it.
    // println!("{}", s1);          - this will throw an error
    // This is similar to what is known as a shallow copy or what happens when you copy just the reference,
    // not the data when programming. Because Rust invalidates the first variable, this is actually known
    // as a *move* (s1 was moved into s2). Now that s2 is the only valid reference to our data, it alone
    // will free the memory. Rust will never automatically create deep copies because of this, copying can
    // be assumed to be inexpensive for runtime preformance.

    // How to deeply copy something in Rust:
    let s1 = String::from("Let's make a deep copy!");
    let s2 = s1.clone();    // This CAN be expensive
    println!("s1 = {}, s2 = {}", s1, s2);

    // For stack only data, we do not need to call clone. This is because the allocation size of this data
    // is known during compile-time. These values are on the stack and copies are quick to make. After we
    // create the value y, there is no reason for us to invalidate x because they are 'basically deep copies'
    // of eachother, there is no difference between deep and shallow copying here and therefore we can leave
    // out clone.
    
    // There is an annotation called the Copy trait that is mutually exclusive with the Drop trait. Values with
    // the copy trait are as follows: integer types, boolean type, floating point values, character type, tuples
    // if their elements all have the Copy trait.

    // Functions and ownership
    // Passing variable into function will move or copy just like assignment.
    let s = String::from("on da heap");
    takes_ownership(s);     // s gets moved into the function and is invalidated
    // println!("{}", s);     //- Error

    let x = 5;
    makes_copy(x);
    println!("{} after function", x);      //- Valid code

    // Returning values can also transfer ownership and move.
    let s1 = gives_ownership(); // function moves return value into s1

    let s2 = String::from("new s2 string thingy");

    let s3 = takes_and_gives_back(s2); // function moves s2 into function and then function variable into s3

    // This does create some annoyances though, look at this:
    let s1 = String::from("hi there, what is my length?");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
    // This is way to complicated for simple calculations like this, why pass back variable ownership and blah
    // blah blah. There is a better way.
}

fn takes_ownership(some_string: String) {
    println!("{}!", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}!", some_int);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string     // Moves ownership to calling point of program.
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string        // Moves into function, then moves out of function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}