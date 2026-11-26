mod hello_world;
mod variable;

fn main() {
    hello_world::hello_world();
    variable::variable();
    variable::string();
    println!("{}", variable::reverse_word(String::from("Hello World")));
}
