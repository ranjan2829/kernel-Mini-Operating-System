pub trait Summary{
    fn summarize(&self)->String;

}
pub struct NewsArticle{
    pub healine :String,
    pub location: String,
    pub author: String,
    pub content :String,

}
impl Summary for NewsArticle{

}
fn main{

}
