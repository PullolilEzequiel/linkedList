pub struct LinkedList<T: Clone> {
    head: Option<Box<LinkedNode<T>>>,
    end: Option<Box<LinkedNode<T>>>,
}

struct LinkedNode<T: Clone> {
    element: T,
    next: Option<Box<LinkedNode<T>>>,
    size: u32,
}
