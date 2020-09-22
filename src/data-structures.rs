///COMPOUND TYPES///
fn compound_types() {
    // Tuple type -> elements of different type, Fixed length.
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Array type -> elements of the same type, Fixed length.
    let a = [1, 2, 3, 4, 5];


}

fn another_function(x: i32, y: i32) -> i32 {
    println!("value of x: {}", x);
    println!("value of y: {}", y);

    x // return is omitted
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn loops() {
    // infinite loop till you stop it
    loop {
        println!("again!");
    }

    // loop with condition
    let mut i = 0;
    while i < 10 {
        println!(i);
        i += 1;
    }

    // loop over a collection
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // reverse for loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn structs() {
    struct User {
        username: String,
        email: String,
    }

    let user = User{
        email : String::from("leo.braga95@gmail.com"),
        username: String::from("bragaz"),
    };


    fn buildUser(username: String, email: String) -> User {
        User {
            username,
            email,
        }
    }

    // struct update syntax ..
    let user2 = &mut User {
        ..user
    };
}