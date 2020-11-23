pub fn greet_message() -> String {
    String::from("Hello, Rust!")
}

pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_message_is_hello_rust() {
        assert_eq!("Hello, Rust!", greet_message());
    }

    #[test]
    fn test_add_should_add_left_and_right() {
        let left = 1;
        let right = 2;
        assert_eq!(left + right, add(left, right));
    }

    #[test]
    fn test_add_should_not_muoltiply_left_and_right() {
        let left = 1;
        let right = 2;
        assert_ne!(left * right, add(left, right));
    }
}
