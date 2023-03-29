// rust里面的每个引用都有自己的
// 生命周期

// 生命周期就是让引用保持有效的作用域

//大多数情况，生命周期是隐式的，可被推断的
// 当引用的生命周期可能以不同的方式互相关联时
// 手动标注生命周期

// 存在的主要目的-避免悬垂引用

// error
// rust编译器的借用检查器：比较作用域来判断所有的借用是否合法
// x是被引用者 被引用的生命周期比引用者的短而报错
/*
 --> life_cycle\src\main.rs:16:17
   |
16 |             r = &x;
   |                 ^^ borrowed value does not live long enough
17 |         }
   |         - `x` dropped here while still borrowed18 |         println!("r: {}", r)
   |                           - borrow later used here
 */
// fn main() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x; //
//         }
//         println!("r: {}", r)
//     }
// }

// 泛型生命周期
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let res = longest(string1.as_str(), string2);
    println!("The longest string is {}", res)
}
// error2
/*
 --> life_cycle\src\main.rs:45:30
   |
45 | fn longest(x:&str,y:&str) -> &str{
   |              ----   ----     ^ expected named lifetime parameter
 */
/*
missing lifetime specifier
this function's return type contains a borrowed value, but the signature does not
say whether it is borrowed from `x` or `y`
*/

// fn longest(x:&str,y:&str) -> &str{
//   if x.len() > y.len(){
//     x
//   }else{
//     y
//   }
// }

// 改 告诉编译器 参数和返回结果 生命周期都是a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
