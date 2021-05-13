// An example of the borrow checker looking for reference lifetimes...

use std::fmt::Display;

fn main() {
    {
        let r;

        {
            let x = 5;
            r = &x;
            
            println!("r: {}", r);
        }

        let str1 = "abcd";
        let str2 = String::from("mystr");
        println!("The longest string is {}", longest(str1, str2.as_str()));
    }


    let string1 = String::from("thisisstringone");
    let result;
    {
        let string2;

        string2 = String::from("thisISstringTWOforSure");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);

        // string2 is valid until here
        // result is a reference to something that is valid until here
    }
    // Doing this would make the code not compile, as references to result are not valid here
    // println!("The longest string is {}", result);
    // The error that pops us though reminds us that the borrowed value of result does not live long enought in line 25.
    // And that 'string2' is dropped while still being borrowed (need it to be valid till end of outer scope.)


    // string1 is valid until here


    // This is where the section begins for structs
    let novel = String::from("Call me Ismael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // This instance of ImportantExcerpt cannot outlive the reference inside.

    // Note that not every function working with references as params and return type need these lifetime parameters,
    // even thought every reference has them, instead some lifetime references are built into the compiler as they
    // are so deterministic and common that the tedium removed is worth it. See the first_word function from ch04-09
    // There are called lifetime elision rules. 

    // STATIC LIFETIME Rules
    // Lifetime meaning that the value *can* live for the entire program. For example, string literals!
    let s: &'static str = "This has a static lifetime";

    // GENERIC Type parameters, trait bound and lifetimes
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display{
        println!("Announcment!: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Note that lifetimes are a type of generic, so they belong with the other generic arguments.
}

// We must impose the constraint that all these references must have the same lifetime. We can do that by giving it
// an explicit lifetime generic in the signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Mutable reference with explicity lifetime.
// &'a mut i32 foo;

// The output string will also have a lifetime that is at least as long as 'a
// It will have the same lifetime as the smaller of the lifetimes passed in. 

// An important note straight from the book itself:
// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which 
// are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string 
// slice returned from the function will live at least as long as lifetime 'a.

// Rust finds the lifetime that gets passed with a reference into a function by observing the concrete lifetimes
// of each of the parameters which is defined by their respective scopes, and then finding the intersection of
// those lifetimes or where they overlap.

// See Line 19 for more

// If we had changed the function body for longest so that it could only return 1 paramter, the first, we would not need
// to specify a lifetime for the second parameter.
fn longest_bad<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// If we were to create a function that creates a value inside of it and then passes it back out to the caller. Since the
// ownership of the object created inside the function is never tranferred to the outside scope, as soon as the function
// finishes, the value will be dropped. There is no way with specifying lifetime parameters in order to get around this.
// Consider the following.

// fn longest_illegal<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// We cannot specify a lifetime for our parameters and our returned value that will make this a valid function.
// The best way to fix this would be to return an owned data type to the calling function instead of a reference
// so that the calling function can be responsible for dropping the value when appropriate.

// This is all to avoid dangling pointers, and ensure memory safety.


// The next half of this ection is about structs and lifetimes so that structs can hold pointers.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}