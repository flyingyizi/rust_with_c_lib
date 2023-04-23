#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// extern "C" {
//     pub fn say_hello();
// }
include!(concat!(env!("OUT_DIR"), "/clibsample_bindings.rs"));

fn main() {
    // println!("Hello, world!");
    unsafe { say_hello() };
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {}
}
