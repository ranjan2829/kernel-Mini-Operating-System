use std::io;
fn main(){
    println!("guess the number");
    println!("input the number guessed");
    let mut guess=String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("you guessed : {guess}");
}