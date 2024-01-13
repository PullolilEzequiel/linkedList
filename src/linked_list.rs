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

/*
  Observacion:
    - El campo middle esta pensado para reducir los costos de recorridos a
    la mitad
*/
struct LinkedNode<T: Clone> {
    element: T,
    next: Option<Box<LinkedNode<T>>>,
}
pub struct LinkedList<T: Clone> {
    head: Option<Box<LinkedNode<T>>>,
    middle: Option<*mut LinkedList<T>>,
    end: Option<*mut LinkedList<T>>,
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
}
