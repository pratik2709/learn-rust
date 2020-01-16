use std::mem;

pub struct List{
    head: Link
}

#[derive(PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(PartialEq)]
enum Link {
    Empty,
    More(Box<Node>)
}

impl List{
    pub fn new() -> Self{
        List {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem:i32){
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        //todo : mem replace
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

//    pub fn disp(&mut self) {
//        let newPtr = self.head;
//        while self.head != Link::Empty {
//            println!("{}" , 1);
////            self.head = self.head.next
//        }
//    }

}

mod test {
    use super::List;
    #[test]
    fn basics() {
        // TODO
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
//        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}