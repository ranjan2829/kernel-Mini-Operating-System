pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    // Example usage
    let article = NewsArticle {
        headline: String::from("Breaking News"),
        location: String::from("New York"),
        author: String::from("John Doe"),
        content: String::from("Lorem ipsum dolor sit amet"),
    };
    println!("News Article: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("alice"),
        content: String::from("Hello, Twitter!"),
        reply: false,
        retweet: false,
    };
    println!("Tweet: {}", tweet.summarize());
}
