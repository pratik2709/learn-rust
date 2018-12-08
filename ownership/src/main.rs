fn main() {
    let s = String::from("hello");
    println!("{} cross",s);
    take_ownership(s);

    let x=5;
    makes_copy(x);
    println!("Hello, world!");
}

fn take_ownership(some_string: String){
    println!("{}", some_string);

}

fn makes_copy(num: u32){
    println!("{}", num);
}
