#[cfg(test)]
mod tests {
    use conversion::tostring::Name;

    #[test]
    fn test_name_to_string_full_name() {
        let name1 = Name {
            first: String::from("John"),
            middle: String::from("C"),
            last: String::from("Doe"),
        };
        assert_eq!("John C Doe", name1.to_string());
    }

    #[test]
    fn test_name_to_string_empty_string() {
        let name1 = Name {
            first: String::from(""),
            middle: String::from(""),
            last: String::from(""),
        };
        assert_eq!("", name1.to_string());
    }

    #[test]
    fn test_name_to_string_just_first() {
        let name1 = Name {
            first: String::from("John"),
            middle: String::from(""),
            last: String::from(""),
        };
        assert_eq!("John", name1.to_string());
    }

    #[test]
    fn test_name_to_string_first_and_last() {
        let name1 = Name {
            first: String::from("John"),
            middle: String::from(""),
            last: String::from("Doe"),
        };
        assert_eq!("John Doe", name1.to_string());
    }

    #[test]
    fn test_name_to_string_only_last() {
        let name1 = Name {
            first: String::from(""),
            middle: String::from(""),
            last: String::from("Doe"),
        };
        assert_eq!("Doe", name1.to_string());
    }

    #[test]
    fn test_name_to_string_only_middle() {
        let name1 = Name {
            first: String::from(""),
            middle: String::from("C"),
            last: String::from(""),
        };
        assert_eq!("C", name1.to_string());
    }
}
