mod collections;
mod errors_handling;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    // Associated function
    fn new_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit(),
    Move{x: u32, y: u32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

//Impl can be done even on enum types
impl Message {
    fn call(&self) {

    }
}

fn OptionEnum() {
    let some_number = Some(1);
    let some_text = Some("yo");

    let absent_number : Option<i32> = None;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn matching_with_Values(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn matching_with_Optional(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i +1),
    }
}

fn matching_with_placeholder(){
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn main() {
    let five = Some(5);
    let six = matching_with_Optional(five);
    let none = matching_with_Optional(None);
}


