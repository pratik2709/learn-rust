include!("post.rs");

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch time");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch time", post.content())

}


