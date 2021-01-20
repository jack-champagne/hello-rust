fn main() {
    
    // This section looks really interesting about strings and utf-8 and the such. I remember reading
    // an article long ago that detailed on the intricacies of internationalization and strings. Mainly
    // I remember how complicated the article pointed out, it can be to describe characters of a language.
    // Some characters in UTF-8 require more bits or something like that

    // Vec and String share a lot of similarities as they are both collections, here are a few examples
    let mut s = String::new();

    // String literals and to_string
    let data = "my new string"
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
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

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

    

}
