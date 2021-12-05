pub struct IntoIter<T>(List<T>);


impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct List<T> {
    head: Link<T>
}
