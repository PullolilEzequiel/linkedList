struct LinkedNode<T: Clone> {
    element: T,
    next: Option<Box<LinkedList<T>>>,
}

pub struct LinkedList<T: Clone> {}
