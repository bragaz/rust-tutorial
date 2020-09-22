use std::fs::File;
use std::io::ErrorKind;

fn errors_with_Result() {
    let f = File::open("hello.txt") ;

    // handle errors without distinguish them
    let f = match f {
        Ok(file) => file,
        Err(err) => panic!("The file doesn't exist {:?}", err),
    };

    // handle different errors with match
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
        _ => {}
    };

    // handle errors with closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // handle errors with wrap (returns the object itself or an error)
    let f = File::open("hello.txt").unwrap();

    //handle errors with expect (let you choose the panic message)
    let f = File::open("hello.txt").expect("failed to open the file");


}

use std::io;
use std::io::Read;

fn errors_propagation() -> Result<String, io::Error>{
    let f = File::open("hello.txt");

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

fn errors_handling_with_operator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}