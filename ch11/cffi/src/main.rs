extern "C" {
    fn c_fib(n: u32) -> u32;
}

fn main() {
    println!("Hello, world!");
    unsafe {
        println!("{}", c_fib(45));
    }
}
