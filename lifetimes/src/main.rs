use std::mem;
include!("static.rs");

fn main(){
    execute();
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32{
    *i + *j
}

struct MutableIterator<'iter, T>{
    slice:&'iter mut [T]
}

impl <'iter, T> Iterator for MutableIterator<'iter, T>{
    type Item = &'iter mut T;
    fn next<'next>(&'next mut self) -> Option<Self::Item>{
        let mut slice1 = mem::replace(&mut self.slice, &mut []);
        let (first, last) = slice1.split_first_mut()?;
        self.slice = last;
        Some(first)
    }
}