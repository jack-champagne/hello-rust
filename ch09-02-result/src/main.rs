use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;

fn main() {
    // We know that f is of type Result<std::fs::File, std::io::Error>
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error)
    // };

    // Adding more functionality to error handling
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("There was a problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // This is great! But there is a lot of match expression and not only that but there is
    // a better way that is discussed later -- see listing 9-5 and chapter 13 to read about closures

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });
    // These are what are called closures

    // unwrap helper method (on Result<T, E>) is very useful for obtaining the inner part (if it
    // exists) and calling panic! automatically if it doesn't

    let f = File::open("filedoesnotexist.txt").unwrap(); 
    // If this file does not exist, then No such file or dir error will occur 

    // expect is just like the unwrap method but it allows for the specification of the error message
    let f = File::open("expectthistothrowaninterestingerror.txt").expect("Interesting error!");
}

// Errors can propogate by returning the error to the calling code as so
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username-file.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// There is a better (and more concise way) to do this error propogation using just one symbol: ?
fn read_username_from_file_better() -> Result<String, io::Error> {
    let mut f = File::open("username-file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// One important difference between using the ? operator and breaking things out into match statements
// is that the ? op will automatically do a type conversion to get the correct type of error as defined
// in the function signature. This way, multiple cases (with different error types) can be handeled.

// An even short example using the chaining now possible (legible?) from ? 
fn chained() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("testfile.txt")?.read_to_string(&mut s)?;
    
    Ok(s)
}

// MORE POWER!!!
fn read_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}