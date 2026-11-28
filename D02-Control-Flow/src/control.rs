use std::io::{self, Write};

fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn number_input(prompt: &str) -> i32 {
    loop {
        let text = input(prompt);
        match text.parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("{text} is not a number. try again")
        }
    }
}

pub fn ifelse() -> () {
    const MINIMUM: i32 = 70;
    let result = number_input("Number: ");

    if result >= MINIMUM {
        println!("Passed");
    }
}

pub fn even_odd() -> () {
    let number = number_input("Input Number: ");
    println!("{}", if number % 2 == 0 {"Even"} else {"Odd"});
}

pub fn do_while() -> () {
    let data: [i32;5] = [1,2,3,5,4];
    let mut index = 0;
    while index < data.len() {
        println!("Data: {}", data[index]);
        index += 1;
    }
}

pub fn looping() {
    let count = number_input("input length: ");
    for i in 1..count + 1 {
        if i % 2 == 0 && i % 3 == 0 {
            println!("FizzBuzz");
        } else if i % 2 == 0 {
            println!("Fizz");
        } else if i % 3 == 0 {
            println!("Buzz");
        } else{
            println!("{i}")
        }
    }
}

pub fn triangle() {
    let height: i32 = 5;

    for i in 1..=height {
        for _ in (1..=i).rev() {
            print!("*");
        }
        println!();
    }
}

fn new_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut new = String::new();
    io::stdin().read_line(&mut new).unwrap();
    return new.to_string();
}

// pub fn time_table() {
//     let number: i32 = number_input("Input number: ");
//     for i in 1..11 {
//         println!("{} x {} = {}", number, i, number * i);
//     }
// }