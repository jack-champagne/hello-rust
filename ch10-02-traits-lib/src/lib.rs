fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you already know..."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize()); // Default will give (Read more...) 
    // since there is no specific implementation.

    // Since traits ensure implementation of certain methods, we can leverage that on any
    // values that implement the trait for example:
    let x = 5.to_string();
}

// We can also implement default behaviour be specifying a body to implemented functions.
pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more by {}...)", self.summarize_author())
    }
    
    // Default implementations can call on other methods in the same trait
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    // To implement the default behaviour, we can spec an empty impl block like this.

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// It is also possible to accept a trait as a parameter type to have a function operate
// on many types that all share the same trait
fn notify(item: &impl Summary) {
    println!("Breaking! {}", item.summarize());
}

// This idea is shorthand for this:
fn notify_1<T: Summary>(item: &T) {}

// Keep in mind that if we had multiple variables coming into our function, to use this trait bound
// can be very useful
fn notify_2<T: Summary>(item: &T, item2: &T) {}

// In the above line of code, both parameters must implement that Summary trait (because of trait bound)
// but they are also constrained to being of the same type! I could not call with one paramter
// being a Tweet and the other being a NewArticle. They must be the same.

fn notify_3<T: Summary, G: Summary>(item: &T, item2: &G) {} // Two different types implementing trait
fn notify_3_1(item: &impl Summary, item2: &impl Summary) {} // Two different types of summary trait again
// Same as line before

// We can specify more than one trait bound. as so
fn notify_4(item: &(impl Summary + Display)) {}
// Now item must implement Summary and Display traits, equivalent
fn notify_4_1<T: Summary + Display>(item: &T) {}

// Sometimes it can be useful to seperate Traits from the function signature for readability
fn function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn functionv2<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
}

// When returning types the implement a trait:
fn ret_summarizable() -> impl Summary {
    Tweet {
        username: String::from("jackc"),
        content: String::from("This is a really short tweet"),
        reply: false,
        retweet: false,
    }
}

// BUT! This does not work for function that actually can return just more than one type the implements
// a trait
// The following code doesn't compile.
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

// A new largest function that we could define is so
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest { // Here the trait std::cmp::PartialOrd would be useful.
            largest = item;
        }
    }

    largest
}
// See the ch10-01-generics file for another implementation of this same function, but with ownership
// rules changed.
// Trait bounds can be used to 'conditionally implement methods'
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    } 
}

impl<T: Display + PartialOrd>Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}