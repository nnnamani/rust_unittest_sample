pub fn greet_message() -> String {
    String::from("Hello, Rust!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_message_is_hello_rust() {
        assert_eq!("Hello, Rust!", greet_message());
    }
}
