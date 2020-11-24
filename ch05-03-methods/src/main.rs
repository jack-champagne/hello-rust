// Methods are declared similarly to functions with the keyword fn
// They are always defined in the context of a struct and will have
// self as their first parameter.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl is the keyword that defines an implementation block for rectangle.
// Now when we call this from main, we can use method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2 : &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }

    // Associated functions: These functions are functions that are associated with
    // the struct but do *not* take in self as a parameter. String::from is an example
    // of an associated function for example.
    // They are often used for constructors that return a new instance of the struct based on
    // the passed in parameters.
    // To call this function we use the :: syntax as before. This function is *namespaced* by a
    // struct. 

    fn square(edge_length : u32) -> Rectangle {
        Rectangle {
            width: edge_length,
            height: edge_length,
        }
    }

}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq1 = Rectangle::square(1);
    println!("SQ1 Area!: {}", sq1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // Insead of area(&rect1)
    );
}

// It is important to note the ownership pattern again here.
// If we wanted to take ownership of the variable (rare because
// use cases usually include transforming original variable and
// making sure it cannot be used again after), we would've typed
// just self in the method declaration for area. Instead, we typed
// &self because we just want to borrow self, and only read the data
// within, *not write*. If we wanted to write, we would've typed
// &mut self for a mutable self.

// Another good note includes how we interact with the methods of pointers
// versus values. In c++ for example, -> is used to call a method on a pointer.
// This avoids dereferencing it first and then calling the method attached to the
// value (object). This is done, automatically in rust as the compiler can understand
// based on the calling context and the method signature what the user
// is trying to do (calling method of reference/pointer or calling method of object/value)

// It is also of note that multiple impl blocks are allowed for a single struct.
// This will be useful later when generics types and traits are introduced

// Next up, Rust enums! 