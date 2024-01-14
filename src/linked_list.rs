/*
  INV.REP: Dado un LinkedList conlos campos 'head', 'middle', 'size' y 'end'
  - Size nunca puede ser < a 0
  - Size representa la cantidad de nodos que hay desde 'head' hasta 'end'
  - Si size es igual a 0 entonces head y end deben ser None
  - Los campos 'head' y 'middle' no pueden ser None sí 'head' es Some
    * Some y None son los constructores de Option
  - El LinkedNode de 'end' no puede tener como 'next' al nodo de 'head'
  - El LinkedNode de 'middle' no puede esta enlazado a head'
*/
#[derive(Clone)]
struct LinkedNode<T: Clone> {
    element: T,
    next: Option<Box<LinkedNode<T>>>,
}
pub struct LinkedList<T: Clone> {
    head: Option<Box<LinkedNode<T>>>,
    middle: Option<Box<LinkedNode<T>>>,
    end: Option<Box<LinkedNode<T>>>,
    size: u32,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            middle: None,
            end: None,
            size: 0,
        }
    }
    pub fn len(&self) -> u32 {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// Replace the first element
    pub fn push(&mut self, elem: T) {
        if self.head.is_none() {
            let new_node = Some(Box::new(LinkedNode {
                element: elem,
                next: None,
            }));
            self.head = new_node.clone();
            self.middle = new_node.clone();
            self.end = new_node;
        } else {
            self.head = Some(Box::new(LinkedNode {
                element: elem,
                next: self.head.clone(),
            }));

            // calculate the middle element
            if self.size % 2 == 0 {
                if let Some(n) = &self.middle {
                    self.middle = n.next.clone();
                }
            }
        }
        self.size += 1;
    }

    /// Return the element at index 'index'
    /// Precond: Set size is >= index
    pub fn get_at(&self, index: u32) -> Result<T, String> {
        let mut i = index;
        let mut node: &Option<Box<LinkedNode<T>>> = &self.head;
        let half_size = self.size / 2;
        if index == self.size {
            node = &self.middle;
            i = 0;
        } else if index > half_size && half_size % 2 == 0 {
            i -= half_size;
            node = &self.middle;
        }
        while i > 0 {
            if let Some(n) = node {
                node = &n.next;
            }
            i -= 1;
        }

        match node {
            Some(n) => Ok(n.element.clone()),
            None => Err(format!("Nonexistent element in index {}", index)),
        }
    }
}
