struct List<T>{
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    value: T,
    next: Link<T>
}

impl<T> List<T>{
    fn new()-> Self{
        List{
            head: None
        }
    }

    fn push(&mut self, val: T){
        let node = Box::new(Node{
            value: val,
            next: std::mem::replace(&mut self.head,None)
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<T>{
        match std::mem::replace(&mut self.head, None){
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}


#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basics(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}