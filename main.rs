mod factorial;
mod threads_exmaple;

fn main(){
    println!("\n");
    println!("Factorial Exmaple:");
    factorial::factorial();
    println!("\n");
    println!("Thread Example:");
    threads_exmaple::thread_example();
}