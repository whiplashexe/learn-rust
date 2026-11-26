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