#[cfg(test)]
mod test {
    use linkedList::LinkedList;

    #[test]
    fn is_empty_test() {
        let mut list = LinkedList::<u32>::new();
        assert!(list.is_empty() == 1);
    }
    #[test]
    fn insert_test() {
        let mut list = LinkedList::<u32>::new();
        list.insert(10);
        assert!(list.len() == 1);
    }
}
