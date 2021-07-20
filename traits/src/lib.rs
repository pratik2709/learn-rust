pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, {}", self.headline, self.location)
    }
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}, {}", self.content, self.username)
    }
}