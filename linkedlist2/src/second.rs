struct List{
    head: Link
}

type Link = Option<Box<Node>>;

struct Node{
    value: i32,
    next: Link
}

impl List{
    fn new(&mut self)-> Self{
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
}