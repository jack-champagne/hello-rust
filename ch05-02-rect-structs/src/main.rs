// This is a project that shows how one could use structs in a program.
// EDIT 0: To look at why this program is written the way it is, look
// in the commit history for this file as it has been changed over time
// to reflect the refacorings that have taken place to use structs.
// EDIT 1: The same program has been implemented with tuples, but width
// and height values are unnamed, which makes it confusing!

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 15,
        height: 20
    };

    println!(
        "The area of the rectangle is {} square pixels!",
        area(&rect1)
    );

    // We want the above to pass a reference to rect because we want main
    // to retain ownership, so that this next bit of code is callable.
    println!(
        "The height element of rect is {}",
        rect1.height
    );

    // What if we want to print our rectangle to stdout?
    println!(
        "rect1 is {:#?}",
        rect1
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

// The #[derive(Debug)] trait that we have added to rectangle is greatly useful
// and can be used on any struct. More at appendix C. More about this in chp 10.
// Now, what if we could bind this area method specifically to rect types? Next!