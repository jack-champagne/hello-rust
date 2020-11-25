// Enumerators: an example use case... IPV4 vs IPV6

// We can define an enumerator as so using the enumerator keyword and listing out
// the possible values it can take.
enum IPAddrKind {
    V4,
    V6,
}

struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

// We can create instances of the IPAddrKind using the :: syntax.
// Both four and six are of type IPAddrKind which means we can
// create method signatures per below main.
fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    route(four);
    route(six);

    // These two code blocks below represent a chapter 5 way of doing this.
    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };

    // There is another way. A simpler way of representing structs in Rust is
    // to put data directly into each enum variant. Like as below
    enum IPAddr {
        V4(String),
        V6(String),
    }

    let home = IPAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddr::V6(String::from("::1"));

    // We can also have multiple data attached to a struct value
    enum IPAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IPAddr2::V4(127, 0, 0, 1);
    let loopback = IPAddr2::V6(String::from("::1"));

    // Wow! It looks like the standard library has a definition of IpAddr that we can
    // use. We can put any object type as the data to any enum variant.

    // We can also use impl block on enums as well!
    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // Anonymous inner struct
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // Do call and define method body.
        }
    }

    let m = Message::Write(String::from("Hello world!"));
    m.call();

    // Another example of an enum from the standard library that is
    // also useful. The Option enum.
    // This is useful because the 'null' value does not exist in Rust.
    // Instead, a different implementation of this concept is as so:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // This is even included in the prelude... there is no need to bring it into scope at all.
    // Including its variants (Some, None)
    // Here are some examples (No pun intended)

    let some_number = Some(6);
    let some_string = Some(String::from("test string"));

    let absent_number: Option<i32> = None; 
    // We need to tell Rust compiler what the type is of absent number still, which is done with
    // the syntax in the above line.

    // Let's try and use the option type...
    let x : i8 = 5;
    let y : Option<i8> = Some(5);

    // let sum = x + y; // This line will not compiler correctly.

}

// This function will take a parameter of type IPAddrKind. Check main
// to see this function called.
fn route(ip_kind : IPAddrKind) {

}