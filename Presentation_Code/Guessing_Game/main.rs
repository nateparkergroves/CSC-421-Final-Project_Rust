
use std::io::{stdin,stdout,Write};
use rand::*; // 0.8.5


pub fn main() {
   let answer = rand::thread_rng().gen_range(1..11);
   println!("Welcome to the game!");
   let mut counter = 0;
   loop{
   counter += 1;
   let guess = get_user_input();
   println!("Your guess was {}", guess);
   if guess < answer {
       println!("Too low, try again");
       continue;
   }else if guess > answer{
        println!("Too high, try again");
        continue;
   }else{
       println!("You won! It took {} guesses.", counter);
       break;
   }
   }
   
}


fn get_user_input() -> u32 {
    loop {
        println!("Please enter a number 1-10:");
        let _ = stdout().flush();

        let mut s = String::new();
        stdin().read_line(&mut s).expect("Failed to read line");

        match s.trim().parse::<u32>() {
        Ok(num) if num >= 1 && num <= 10 => return num,
        Ok(_) => println!("Number must be between 1 and 10."),
        Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
