pub fn test() {
    println!("Hello, world! This is from Kagero, the library.");
}

#[cfg(feature = "printer")]
pub mod printer;

#[cfg(feature = "orochi")]
pub mod orochi;