use libc::c_long;

mod bindings;

fn main() {
    let x1=5;
    let x2=5;
    unsafe { 
      let x=bindings::Add(x1,x2); 
      println!("Add={}",x);
    }
    println!("Ran the unsafe code.");
}
