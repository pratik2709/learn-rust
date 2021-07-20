include!("lib.rs");

fn main() {
    let tweet = Tweet{
        username: String::from("twttr"),
        content: String::from("first tweet ever")
    };

    println!("Summarize tweet: {}", tweet.summarize());
}
