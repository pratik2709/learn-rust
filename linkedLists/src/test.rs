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

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|val|{
            *val = 42;
        });
        assert_eq!(list.peek(), Some(&42));

        assert_eq!(list.pop(), Some(42));
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

    #[test]
    fn into_iter(){
            let mut list = List::new();
            list.push(1); list.push(2); list.push(3);

            let mut iter = list.into_iter();
            assert_eq!(iter.next(), Some(3));
    }
}