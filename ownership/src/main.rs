fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello world");
    let s3 = takes_and_gives_back(s2);

    let x=5;
    makes_copy(x);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(some_string: String) -> String{
    some_string

}

fn makes_copy(num: u32){
    println!("{}", num);
}
