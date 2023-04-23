extern "C" {
    pub fn say_hello();
}

fn main() {
    println!("Hello, world!");

    unsafe {say_hello()};
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {}
}
