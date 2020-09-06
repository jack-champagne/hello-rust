// This is a project that shows how one could use structs in a program.
// EDIT 0: To look at why this program is written the way it is, look
// in the commit history for this file as it has been changed over time
// to reflect the refacorings that have taken place to use structs.
// EDIT 1: The same program has been implemented with tuples, but width
// and height values are unnamed, which makes it confusing!
fn main() {
    let rect1 = (15, 20);

    println!(
        "The area of the rectangle is {} square pixels!",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}