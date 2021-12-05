//shared mutable references cannot co-exist

pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T>{
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node)
        }
    }
}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }

}