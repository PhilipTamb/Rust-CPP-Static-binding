extern crate core;
use core::ffi::c_int;


extern "C"{
    pub fn covert_and_sum( a: c_int, b: c_int) -> c_int;
}


fn main() {
    unsafe{
        let result = covert_and_sum(5, 10);
        println!("[Rust] Result:  {}", result);
    }

}