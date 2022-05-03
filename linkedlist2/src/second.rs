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
        self.head.take().map(|node|{
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node|{
            &node.value
        })
    }

    //returns the first node from the list
    fn iter(&self) -> Iter<T> {
        Iter{
            next: self.head.as_ref().map(|node|{
                &**node
            })
        }
    }
}

//giving stuff one at a time
// immutable iterator instance
struct Iter<'a, T>{
    next: Option<&'a Node<T>>
}

// impl<T> Iterator for Iter<T>{
//     fn new(list: List<T>) -> Self{
//         Iter{
//             next: list.head.map(|node|{
//                 &node
//             })
//         }
//     }
// }




#[cfg(test)]
mod test{
    use super::List;

    #[test]
    fn basics(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}