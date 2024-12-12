mod factorial;
mod threads_exmaple;
mod panic_error;



fn main(){
    println!("Thread Example:");
    threads_exmaple::thread_example();
    println!("\n");

    println!("Factorial Exmaple:");
    factorial::factorial();
    println!("\n");

    println!("Panic vs Error Example:");
    panic_error::panic_vs_error();
    println!("\n");
}


