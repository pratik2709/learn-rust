// include!("lib.rs");

fn main() {
    // let tweet = Tweet{
    //     username: String::from("twttr"),
    //     content: String::from("first tweet ever")
    // };
    //
    // println!("Summarize tweet: {}", tweet.summarize());
}

fn add(a: i32, b:i32) -> i32{
    a+b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(1,2), 3);
        assert_ne!(add(1,2), 4);
    }
}