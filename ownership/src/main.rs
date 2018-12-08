fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello world");
    let s3 = takes_and_gives_back(s2);

    references_and_borrowing();
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(some_string: String) -> String{
    some_string

}


fn references_and_borrowing(){
    let mut s1 = String::from("reference");
    let len = calculate_length(&s1);
    modify_reference_value(&mut s1);
    let len1 = calculate_length(&s1);
    println!("{}, length: {}, l2: {}", s1, len, len1);

}

fn calculate_length(s:&String) -> usize{
    s.len()
}

fn modify_reference_value( s:&mut String){
    s.push_str("__multiverse");
}
