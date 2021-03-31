fn main() {
    let number_list = vec![34, 65, 12, 775, 533, 12, 24];

    let result = largest(&number_list);
    println!("The largest number was {}", result);

    let char_list = vec!['y', 'm', 'c', 'a'];
    
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let int_point = Point {x: 5, y: 10};
    let flt_point = Point {x: 5.0, y: 10.0};

    // Something that won't work because both fields are of the same type is like this:
    // let BAD_point = Point {x: 10, y: 5.0};

    // We could do something like that with mixed point. Note that the two data types do not have
    // to be the same, and they do not have to be different. They can for both.
    let mxd_p1 = MixedPoint {x: 1, y: 2};
    let mxd_p2 = MixedPoint {x: 1.0, y: 3};
    let mxd_p3 = MixedPoint {x: 1, y: 3.0};
    let mxd_p4 = MixedPoint {x: 4.0, y: 3.14 };

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// When we define a generic, we give it a label as we would for the type in functions, inside, etc.
// For example:
// A new largest function that we could define is so
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest { // Here the trait std::cmp::PartialOrd would be useful.
            largest = &item;
        }
    }

    largest
}

// It is also possible to have structs with generic types in their fields as so:
struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// Helpful tip: If the codebase is requiring a lot of generics and is getting hard to read,
// it is likely the code needs to be broken down into smaller bits.

// Generics work in enums as well
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// This is what it would look like in a method definition:
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also define type specific methods for certain data types, here it is for Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Another example of generics in functions and arguments
impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

