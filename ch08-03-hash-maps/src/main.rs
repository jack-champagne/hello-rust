fn main() {
    use std::collections::HashMap;
    // Creating a hash map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Since they are the least popularly used collection in the standard library, there is less support
    // for them. They are not automatically included in the prelude for example and they also do not
    // have a built-in macro to construct them.

    // Just like vectors, hash maps store their data on the heap. Like vectors, they are homogenous.
    // Their keys and values must all be of the same type.

    // Here is a cool example of using some iterator API in order to create a hash map out of a vector of
    // tuples:

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // The HashMap<_, _> type annotation is needed because it is possible to collect into many different
    // data structures. Using the underscores, Rust can infer the type of the keys and values.alloc

    // For types that implement the Copy trait (like i32) values are copied and ownership is not moved.
    // For owned values (like String) values are moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{}", field_name); // Will cause borrow checker to get angry and this will not compile!
    // Value is moved and we try to borrow

    // If we inert references into a hash map, the vales won't be moved into the hash map. The values
    // that the references point to must be valid for at least as long as the hash map is still valid
    // This will be discussed further in chapter 10 in something called Reference Lifetimes

    // Getting a value out of a hash map
    let team_name = String::from("Blue");
    let blue_score = scores.get(&team_name);
    assert_eq!(blue_score, Some(&10));
    let none_score = scores.get(&String::from("Not a valid team"));
    assert_ne!(none_score, Some(&10));
    assert_ne!(none_score, Some(&50));
    assert_eq!(none_score, None);
    // The get method returns an  type (Some or None)

    // Iterating over key value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Handling putting values to the hash map
    // Overwriting a value
    println!("{:?}", scores.get(&String::from("Blue")));    // Prints Some(10)
    scores.insert(String::from("Blue"), 25);                // Overwrites value
    println!("{:?}", scores.get(&String::from("Blue")));    // Prints Some(25)

    // Using the entry API (safe from overwrites)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);

    // Following call adds "Yellow": 50 to the hash map because it does not already exist.
    scores.entry(String::from("Yellow")).or_insert(50);     // Writes parameter to mutable reference
                                                            // for corresponding Entry is it exists
                                                            // if not, it inserts the parameter as the
                                                            // new value for this key and returns a mut
                                                            // reference to the new value
    scores.entry(String::from("Blue")).or_insert(50);       // This call does not change the hash map
    println!("{:?}", scores);

    // Updating a value based on old value 
    let text = "hello world oh wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // Uses a good hasher (not too fast but cryptoographically strong) and can switch to other hashers!
    // -- BuildHasher trait --
    // Some exercises!
}
