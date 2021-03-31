fn main() {
    // A panic! means an unrecoverable error has occured. The stack will unwind and the program will
    // clean up any memory. This can be changed in the cargo.toml under a [profile.release] tag we
    // can put: panic = 'abort'. The resulting binary will be much smaller but the underlying OS will
    // have to clean up the memory the program was using.

    // panic!("Dead in the water");

    let v = vec![4, 5, 6];

    v[99];
}
