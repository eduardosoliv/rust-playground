#[cfg(test)]
mod tests {
    use types::casting::{float_to_int, int_to_char};

    #[test]
    fn test_float_to_int() {
        assert_eq!(65, float_to_int(65.4321_f32));
    }

    #[test]
    fn test_int_to_char() {
        assert_eq!('A', int_to_char(65));
    }
}
