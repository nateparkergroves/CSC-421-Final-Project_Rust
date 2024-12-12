mod rust_sum;
mod threads_exmaple;
mod panic_error;



fn main(){
    println!("Thread Example:");
    threads_exmaple::thread_example();
    println!("\n");

    println!("Thread vs Python Exmaple:");
    rust_sum::thread_sum();
    println!("\n");

    println!("Panic vs Error Example:");
    panic_error::panic_vs_error();
    println!("\n");
}


