use std::mem;
// https://gist.github.com/rylev/3a3dd4b0d8563eb2267f489e559bb70e
include!("static.rs");
// fn main(){
//     execute();
//     let a = 10;
//     let b = 20;
//     let res = add_with_lifetimes(&a, &b);
//     println!("{}", res);
// }
//
// fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32{
//     *i + *j
// }
fn main() {}

struct MutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MutableIterator<'iter, T> {
    type Item = &'iter mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut slice1 = mem::replace(&mut self.slice, &mut []);
        let (first, last) = slice1.split_first_mut()?;
        self.slice = last;
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_iterator() {
        let mut collection = vec![1, 2, 3, 4];
        let mut iter_collection = MutableIterator {
            slice: &mut collection
        };
        println!("{:?}", iter_collection.next());
        assert_eq!(iter_collection.next().unwrap(), &2);
    }
}
