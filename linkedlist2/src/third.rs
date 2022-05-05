struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> List<T> {
    fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next<'b>(&'b mut self) -> Option<&'a mut T> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
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

    #[test]
    fn iter_tests() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        let x = iter.next();
        let y = iter.next();
        println!("{:?},{:?}", x, y);
    }

    #[test]
    fn iter_mut_tests() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        let x = iter.next();
        let y = iter.next();
        println!("{:?},{:?}", x, y);
    }
}