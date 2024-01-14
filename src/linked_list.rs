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
#[derive(Clone, Debug)]
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

            if self.size % 2 == 0 {
                if let Some(n) = &self.middle {
                    self.middle = n.next.clone();
                }
            }
        }
        self.size += 1;
    }
}
