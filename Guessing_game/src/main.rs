use std::io;
fn main(){
    println!("guess the number");
    println!("input the number guessed");
    let mut guess=String::new();
    let apples = 5; // immutable
    let mut bananas = 5; // 
    println!("apple: {apples}");
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("you guessed : {guess}");
}