fn rust_vs_python() {
    let x = Box::new(42);
     println!("Value: {}", x);
 
     let y = x;
     println!("Value: {}", y);
     
     println!("Value: {}", x);
 }

fn main(){
    rust_vs_python();
}