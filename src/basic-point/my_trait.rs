// Trait 告诉编译器
// 某种类型具有哪些并且可以与其他类型共享的功能
// 抽象的定义共享行为
// Trait bounds(约束):泛型类型参数指定为实现了特点行为的类型

// 定义一个Trait
pub trait Summary {
    // 一组行为 可以有多个行为
    fn summarize(&self) -> String;
}

// 获取不同博客的摘要

pub struct NewsArticle {
    pub headline: String,
    pub localtion: String,
    pub author: String,
    pub content: String,
}

// impl NewsArticle 為NewsArticle實現方法
// impl Summary for NewsArticle    為NewsArticle實現實現trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // 使用format宏 将一串字符串组合在一起并进行返回
        format!("{},by {} ({})", self.headline, self.author, self.localtion)
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
        format!("{}:{}", self.username, self.content)
    }
}
