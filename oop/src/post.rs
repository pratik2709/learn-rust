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

    pub fn add_text(&mut self, text:&str){
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str{
        ""
    }
}

struct Draft{

}

trait State{

}

impl State for Draft{

}