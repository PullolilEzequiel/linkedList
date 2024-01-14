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
        assert!(list.len() == 1);
    }
}
