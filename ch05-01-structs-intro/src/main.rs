// Structs allow us to group related data into meaningful objects
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,          // I think this trailing comma is redundant
}

fn main() {
    let firstUser = User {
        email: String::from("jackchampagne.r@gmail.com"),
        username: String::from("jack-champagne"),
        active: true,
        sign_in_count: 1,    // This comma is also redundant
    };

    // The biggest difference between a struct and a tuple is that structs data can
    // be unordered and labeled.
    // We can access elements of a struct using dot notation:
    println!("{}", firstUser.active);

    // Mutable vs immutable structs: An entire struct will either be mutable or immutable
    let mut mutable_user = User {
        email: String::from("mutable@email.com"),
        username: String::from("mutablename"),
        active: true,
        sign_in_count: 1
    };

    // Works
    mutable_user.email = String::from("mutable@jackchampagne.com");
    // Does not work
    // firstUser.active = false;
}

// Heres a cool piece of machinery:
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}

// And theres a nice short-hand for fields of a struct. 'Field init shorthand' It works
// by having the same variable name as the field of the struct.
fn build_user_improved(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

// Theres also another shorthand that is useful. This is called struct update syntax
fn struct_update_syntax() {
    let user1 = User {
        email: String::from("jackchampagne.r@gmail.com"),
        username: String::from("jack-champagne"),
        active: true,
        sign_in_count: 1
    };

    // Lets say we want to create a new struct instance based on the previous user1 instance
    // There are two ways of doing this. This is way 1
    let user2 = User {
        email: String::from("anotheruser@gmail.com"),
        username: String::from("another-user"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };
    // This instance will have active and sign_in_count set the the same values as user1
    // This is WAY 2. And is way better as well.
    let user3 = User {
        email: String::from("cool-shorthand@gmail.com"),
        username: String::from("cool-shorthand"),
        ..user2
    };
    // The above created an instance for the struct for the user 'cool-shorthand' that
    // utilized "struct update syntax" which is the ..user2 at the end.
    // This syntax allows us to specifiy fields in a new struct, and then populate all
    // unspecified fields with the values from a previous struct instance.
}

// Another type of struct, the tuple struct!
fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
// These look like tuples but they are their own unique types. The are instances
// of different structs. Even though the fields in the struct have the same types,
// they have different types because of their differing definitions.
// I cannot use a type Color as a type Point and vice-versa. They are independent.
// This last part above is the biggest reason why they are different from tuples
// Their data is ordered (like tuples) but they are not exchangable. Point and Color
// are different even though they are constructed by the same elements.

// One last type of struct? Unit-like structs. These are structs that do not have
// fields. They behave similarly to the unit type, (). (They are useful for
// implementing trains on a type but do not want to store data in the type itself.)

// We will not have our structs store references unti chapter 10, when we dig into
// lifetime specifiers. This way our structs will always own the data within them.
// Basically, use String intead of &str lol.