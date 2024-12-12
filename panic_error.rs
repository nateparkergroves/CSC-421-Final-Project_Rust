
use std::fs::File;

pub fn panic_vs_error() {
//Panic Example 1
let fruit = "bananas";

assert!(fruit.contains("nan"));
println!("{fruit}");

//Panic Example 2
let x = 10;
let y = 2;
let z = x/y;
println!("{z}");
//Error Example
let f = File::open("main.jpg");  
    //this file does not exist 
    println!("{:?}",f); 

println!("Program ended successfully");
}
