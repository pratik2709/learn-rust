pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post{
    pub fn new() -> Post{
        Post{
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }
}

struct Draft{

}

trait State{

}

impl State for Draft{

}