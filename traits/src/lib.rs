pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, {}", self.headline, self.location)
    }
}
