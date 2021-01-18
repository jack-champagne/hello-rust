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

    {
        let my_mem_vec = vec![0, 7, 19, 112];

        // Can use my_mem_vec
    }   // <-- my_mem_vec goes out of scope here.

    // When a vector gets dropped, so do all the contents of its memory. This gets some asterisks attached when
    // you start dealing with references (and I predict that ownership is what they are talking about here)

    

}
