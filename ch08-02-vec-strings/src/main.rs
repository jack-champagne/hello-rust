fn main() {
    
    // This section looks really interesting about strings and utf-8 and the such. I remember reading
    // an article long ago that detailed on the intricacies of internationalization and strings. Mainly
    // I remember how complicated the article pointed out, it can be to describe characters of a language.
    // Some characters in UTF-8 require more bits or something like that

    // Vec and String share a lot of similarities as they are both collections, here are a few examples
    let mut s = String::new();

    // String literals and to_string
    let data = "my new string";
    let s = data.to_string();

    let s = "my newer string".to_string();

    // From! As discussed before:
    let s = String::from("my next string");

    // Internationalization and UTF-8!
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let hello = String::from("你好");

    println!("{}", hello);


    // Just like a vector, we can add, update, and remove values from a string. We can also concatenate
    // a string using the + operator or the format! macro

    let mut s = String::from("foo string! ");
    s.push_str("bar string!");
    assert_eq!(s, "foo string! bar string!");

    // push_str takes a slice because we don't want to necissarily take ownership of the appended string.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // The push method appends a singular character to the string as so 
    let mut s = String::from("Hell");
    s.push('o');
    assert_eq!(s, "Hello");

    // Concatenation between two strings
    let s1 = String::from("Hello,");
    let s2 = String::from(" world.");
    let s3 = s1 + &s2; //  s1 has been moved because of this and can no longer be used

    // There is a method that gets called when we use the + operator, this seems like it is going to be a
    // idea to the way that c++ allows for operator overloading. 
    // Method signature:
    // fn add(self, s: &str) -> String {}

    // To be precise, the real signature is defined using generics in the std lib

    // Interesting concept, deref coercion, chapter 15. This is where rust will change the &s2 into &s2[..]
    // which is of the right type, &str, to be used by the add method.

    // Note that the s param of the add method is a reference, therefor the method will not take ownership of s.
    // More complex concatenations:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    assert_eq!(s, "tic-tac-toe");

    // Let's instead use the format macro :^)
    // let s = format!("{}-{}-{}", s1, s2, s3); // Won't work because ownership of s1 has been taken by s on line 76.
    let s1 = String::from("tic");   // Now it works
    let s = format!("{}-{}-{}", &s1, &s2, &s3);
    assert_eq!(s, "tic-tac-toe");
    // This also does not take ownership of its parameters!

    // Indexing Strings
    // Since the String class is a wrapper over Vec<u8>, there are some interesting things that we don't and can't do
    let hello = String::from("Hello!");
    // let h = hello[0]  // -- This does not work! Why?

    // len in the String class refers the the number of bytes a string is. hello is 6 bytes long
    assert_eq!(hello.len(), 6);

    // But what if we use some other UTF-8 characters?
    let hello = String::from("Здравствуйте");
    // There are 12 characters here but it takes 24 bytes to store this string!
    assert_eq!(hello.len(), 24);

    assert_ne!("З", "3");
    // The З character (not the same as 3) takes two bytes to store in unicode.

    // Ths sort of code is not allowed as it will return the first byte of the string and *not* the first char
    // This is because some characters take up more bytes in memory than others in UTF-8 encoding.
    // let first_letter = &hello[0];

    // Next section is on Bytes, Scalar Values, and Grapheme clusters
    let hello = String::from("नमस्ते");
    println!("{:?}", hello.as_bytes());
    // The above line will print: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    println!("{:?}", hello.chars());
    // The above will give us something like: ['न', 'म', 'स', '्', 'त', 'े']

    // The reasoning for all of this is mainly because accessing the index of an array should be guarenteed
    // as an O(1) operation and this cannot be guarenteed for strings.

    // Next section: Slices of strings
    let hello = String::from("Здравствуйте");
    let my_slice = &hello[0..4];

    // let my_slice = &hello[0..1];  // This will panic at runtime
    // We can iterate intstead!
    let hello = String::from("नमस्ते");
    for c in hello.chars() {
        println!("{}", c);
    }

    for c in hello.chars() {
        print!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }

    // Getting grapheme clusters is so complex in fact that the standard library does not provide functionality for it

    // Next data structure, hash maps!
}
