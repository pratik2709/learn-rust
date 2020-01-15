use std::mem;

pub struct List{
    head: Link
}

struct Node {
    elem: i32,
    next: Link,
}

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
            next: Link::Empty
        });
        //todo : mem replace
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match &self.head {
            Link::Empty => {

            }
            Link::More(node) => {

            }
        }

        unimplemented!();
    }

}