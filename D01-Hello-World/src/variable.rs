pub fn variable() -> () {
    let name: &str = "Apriana"; // -> cannot replaced 
    let mut hobby: String = String::from("Guitar"); // -> another version of string type
    let mut age: i32 = 23; // -> can be replaced
    println!("Hello, my name is {name}, im {age} years old!. my hobby is {hobby}");
    age = 24;
    hobby = String::from("Fishing");
    println!("Happy birth day!!!. {name} now im {age} years old. My hobby is {hobby}");
}

pub fn string()->() {
    let mut name = String::from("Gede ");
    name.push_str("Apriana");
    println!("String: {name}");

    let mut trim = "  Belajar rust itu seru  ";
    trim = trim.trim();
    println!("trim: {trim}")
}

pub fn reverse_word (s: String)->String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}