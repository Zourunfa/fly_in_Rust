// use std::{fmt::Display};

// // pub trait Summary{
// //   fn summarize(&self) ->String{
// //     // 默认实现
// //     String::from("(Read more ...)")
// //   }

// // }
// // 默认实现的方法可以调用trait中其他的方法，即使这些方法没有默认实现
// pub trait Summary{
//   fn summarize_author(&self) ->String;

//   fn summarize(&self) ->String{
//     format!("(Read more from {}...)",self.summarize_author())
//   }
// };
// // 想要使用上面方法 需要实现 summrize_author
// impl Summary for NewsArticel{
//   fn summrize_author(&self)->self{
//     format("@{}",self.author);
//   }
// }

// pub struct NewsArticel{
//   pub headline: String;
//   pub locatin:String,
//   pub author:String;
// }

// // impl Summary for NewsArticel{

// // }

// pub struct Tweet{
//   pub username:String;
//   pub reply:String;
// }

// impl Summary for Tweet{
//   // 默认实现的重写实现
//     fn summarize(&self)->String{
//     format!("{}:{}",self.username,self.reply);
//     };
// }

// // Trait作为参数
// // impl Trait 语法： 适用于简单情况
// // pub fn notify(item:impl Summary){
// //   println!("Breaking news! {}",item.summarize());
// // }
// // Trait bound语法 适用于复杂情况
// // pub fn notify<T:Summary>(item:T,item2:T){
// //   println!("Breaking news! {}",item.summarize());
// // }
// // 使用加号 实现多个Trait bound
// // pub fn notify<T:Summary + Display>(item:T){
// //   println!("Breaking news! {}",item.summarize());
// // }

// // pub fn notify<T:Summary+Display,U:Clone + Debug>(a:T,b:U)->String{
// //   format!("Breaking news! {}",a.summarize())
// // }
// // 可以是用where语句简化上面trait实现
// // pub fn notify<T,U>(a:T,b:U)->String
// // where
// //   T:Summary+Display,
// //   U:Clone+ Debug
// //   {

// //     format!("Breaking news! {}",a.summarize())
// // }

// // 使用Trait 作为返回类型
// // impl Trait

// pub fn notify(s: &str) -> impl Summary{
//   // 比如这个只能返回这一种Trait 如果加个if else判断返回
//   // 两种不同类型的Trait的话 会报错
//   NewsArticel{
//     headline: String::from("Penguins win the sdadas"),
//     author:String::from("UUSdsad"),
//     locatin:String::from("Pewruehsad asd ")
//   }
// }
