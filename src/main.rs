mod my_trait;
pub use my_trait::Summary;
pub use my_trait::Tweet;
fn main() {
    let tweet = Tweet {
        username: String::from("af"),
        content: String::from("你好嗎"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())
}
