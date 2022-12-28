use std::{io};



fn main() {
    

     println!("Guess a Number");

     println!("Input a number");

     let mut guess=String::new();

     io::stdin().read_line(&mut  guess).expect("Failed to read num" );

     println!("you Guess : {guess}");

}
