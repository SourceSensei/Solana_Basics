fn main() {
    let str: &str = "hello world";
    let string: String = String::from("Hello world from Rust!");

    let slice = &string[.. 6]; // "Hello"
    slice.len();

    // Check docs for more methods that you can use with strings
}