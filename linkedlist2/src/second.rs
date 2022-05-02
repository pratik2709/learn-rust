struct List{
    head: Link
}

type Link = Option<Box<Node>>;

struct Node{
    value: i32,
    next: Link
}

impl List{
    fn new()-> Self{
        List{
            head: None
        }
    }

    fn push(&mut self, val: i32){
        let node = Box::new(Node{
            value: val,
            next: std::mem::replace(&mut self.head,None)
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<i32>{
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