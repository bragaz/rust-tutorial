fn vectors() {
    let vect : Vec<i32> = Vec::new();

    // vector with macro
    let mut v = vec![1, 2, 3];

    // push elements to it
    v.push(4);

    // access to elements
    let second : &i32 = &v[1];

    match v.get(1) {
        Some(second) => println!(second),
        None => println!("No item at {} index", 1),
    }

    // iterating over values

    // immutable
    for i in v {
        println!(v[i])
    }

    //mutable
    for i in &mut v {
        *i += 50;
    }
}

fn strings() {
    let mut s = String::new();

    // convert literals to string
    let lit = "hello i'm a literal";
    let str = lit.to_string();

    // adding element to a string
    let mut res = str + " plus";
    res = format!("{} and this", res);
    res.push_str(" and also this");

    // slice
    let slice = &res[0..2];

    // iterate in a string
    for c in res.chars() {
        println!(c)
    }
}

use std::collections::HashMap;

fn hash_map() {
    let mut scores = HashMap::new();

    // add elements to hash map
    scores.insert("Yellow", 10);
    scores.insert("Blue", 6);

    // accessing elements
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // iterating over elements
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // replace a value
    scores.insert("Blue", 10);

    // check if a value exist and if not, insert it
    scores.entry("Blue").or_insert(10);

    // creating an hash map from two separate vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // after creation of hash map these two fields will be invalid
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // check how many times a word occurs by using hash map
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }


}