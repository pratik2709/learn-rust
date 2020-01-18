pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T>{
        Iter{
            next: self.head.as_ref().map(|node| &**node)
        }
    }
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            // node.next may be empty ?
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }

}
