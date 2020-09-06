// This is a project that shows how one could use structs in a program.
fn main() {
    let height = 15;
    let width = 20;

    println!(
        "The area of the rectangle is {} square pixels!",
        area(height, width)
    );
}

fn area(h: u32, w: u32) -> u32 {
    h * w
}