// Vectors puts values of the same type next to eachother in memory.

fn main() {
    let v: Vec<i32> = Vec::new();   // Empty vector

    // Here we used a type annotation to tell rust the generic of the Vector class (the data type it will store)
    // This is so common that there is a macro for it in Rust the `vec![]` macro as so
    let v = vec![1, 2, 3];

    // Rust infers the type of the Vector elements from the inputs here to be i32. No type annotation is needed.
    // Let's modify a vector

    let mut my_v = Vec::new();

    my_v.push(5);
    my_v.push(6);
    my_v.push(7);
    // Here Rust is also able to infer the type of the data of the vector, i32, from later usages. We must make 
    // this new vector mutable because we change it whevener we add a new element.
    // Vectors are also freed from memory when they go out of scope (ownership). See below


    // When a vector gets dropped, so do all the contents of its memory. This gets some asterisks attached when
    // you start dealing with references (and I predict that ownership is what they are talking about here)
    {
        let my_mem_vec = vec![0, 7, 19, 112];


        // Can use my_mem_vec
    }   // <-- my_mem_vec goes out of scope here.

    // Reading elements of a vector!!
    let v = vec![1, 2, 3, 4, 5];

    let third_elem: &i32 = &v[2];
    println!("The third element is: {}", third_elem);

    match v.get(2) {
        Some(third_elem) => println!("Third elem exists and is: {}", third_elem),
        None => println!("Third elem does not exist")
    }
    // Just tried playing around with doing an index out of bounds sort of thing and the difference in behaviour
    // looks useful

    // If we try accessing an element that is out of bounds, with the $v[], the thread will panic at 'index out of bounds'
    // Where if we try accessing an element using v.get() since we get an option type, we can handle the None case

    // Ownership and references
    // We cannot take a mutable and an immutable borrow of something in same scope. So borrowing an element of
    // our vector can get tricky depending on what we are doing with it.
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let first = &v[0]; //Immutable borrow

    v.push(7); // Mutable borrow

     // Everything is fine here
    println!("Vectors contents are: {:?}", v);
    // Everything is NOT fine here uncomment below to break code (same immut borrow used later here)
    //println!("First elem is: {}", first); 
    
    // This is all because adding to the vector might cause us to need to reallocate the vector and if that
    // happened (and all the elements were copied over to a new space because there wasn't enough space to
    // put all the elements next to eachother) then we could have a reference that is pointing to deallocated
    // memory, which would be bad.

    // Iterating over values in a vector
    let v = vec![10, 100, 1000, 10000];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![10, 100, 1000, 10000];
    for i in &mut v {
        *i += 10;
    }

    println!["{:?}", &v];
}
