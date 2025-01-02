#[cfg(test)]
mod tests {
    use customtypes::linkedlist::List;

    #[test]
    fn test_len_empty_list() {
        let list: List = List::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut list: List = List::new();
        list = list.append(1).append(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_append() {
        let mut list: List = List::new();
        list = list.append(1).append(2).append(3);
        assert_eq!(list.stringify(), "1, 2, 3, Nil");
    }

    #[test]
    fn test_prepend() {
        let mut list: List = List::new();
        list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.stringify(), "3, 2, 1, Nil");
    }

    #[test]
    fn test_get_empty_list() {
        let list: List = List::new();
        assert_eq!(list.get(1), None);
    }

    #[test]
    #[should_panic]
    fn test_get_or_fail_empty_list() {
        let list: List = List::new();
        list.get_or_fail(1);
    }

    #[test]
    fn test_get() {
        let mut list: List = List::new();
        list = list.append(5).append(8).append(10);

        assert_eq!(list.get(0), Some(5));
        assert_eq!(list.get(1), Some(8));
        assert_eq!(list.get(2), Some(10));
    }

    #[test]
    fn test_get_out_of_bounds() {
        let mut list: List = List::new();
        list = list.append(5).append(8).append(10);
        assert_eq!(list.get(3), None);
    }

    #[test]
    #[should_panic]
    fn test_get_or_fail_out_of_bounds() {
        let mut list: List = List::new();
        list = list.append(5).append(8).append(10);
        list.get_or_fail(3);
    }
}
