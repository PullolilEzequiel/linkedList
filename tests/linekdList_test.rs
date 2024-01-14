#[cfg(test)]
mod test {
    use linkedList::LinkedList;

    #[test]
    fn is_empty_test() {
        let list = LinkedList::<u32>::new();
        assert!(list.is_empty());
    }
    #[test]
    fn push_test() {
        let mut list = LinkedList::<u32>::new();
        list.push(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.get_at(0), Ok(10));
    }

    #[test]
    fn get_at_test() {
        let mut list = LinkedList::<u32>::new();
        for i in 0..10 {
            list.push(i);
        }

        assert_eq!(list.len(), 10);

        assert_eq!(list.get_at(5), Ok(4));
        assert_eq!(list.get_at(9), Ok(0));
        assert_eq!(
            list.get_at(12),
            Err("Nonexistent element in index 12".to_string())
        );
    }
}
