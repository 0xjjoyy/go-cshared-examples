use libc::c_long;

#[link(name = "awesome", kind="dylib")]
extern {
    fn Add(num1: c_long, num2: c_long) -> c_long;
}

fn main() {
    let x1=5;
    let x2=5;
    unsafe { 
      let x=Add(x1,x2); 
      println!("Add={}",x);
    }
    println!("Ran the unsafe code.");
}
