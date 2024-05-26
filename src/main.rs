use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess!!");

    //by default properties/objects defined by let are immutable
    //mut -> mutable(changes value) variable
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {} ", guess);
//continue from top of page 19 rust book
}
