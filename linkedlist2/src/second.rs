struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None
        }
    }

    fn push(&mut self, val: T) {
        let node = Box::new(Node {
            value: val,
            next: std::mem::replace(&mut self.head, None),
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    //returns the first node from the list

}

//giving stuff one at a time
// immutable iterator instance
// just a borrowed value so not needed on the heap therefore no box ?
//next value should be pointing to the next node
struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> List<T>{
    fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| {
                &**node
            })
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| {
                &**node
            });
            &node.value
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
    fn iter_tests(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
    }
}