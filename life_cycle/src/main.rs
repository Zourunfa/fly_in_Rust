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
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let res = longest(string1.as_str(), string2);
//     println!("The longest string is {}", res)
// }
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
//
// fn longest(x:&str,y:&str) -> &str{
//   if x.len() > y.len(){
//     x
//   }else{
//     y
//   }
// }

// 改 告诉编译器 参数和返回结果 生命周期都是a
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 生命周期的标注语法
// 1.生命周期的标注不会改变引用的生命周期长度
// 2.当指定了泛型的生命周期参数，函数可以接收带有任何生命周期的引用
// 3.生命周期的标注，描述了多个引用的生命周期间的关系，但不影响生命周期

// 4. 一 ' 开头，通常全小写且非常短 很多人用 'a

// 生命周期标注的位置
// 在引用的&符号后
//
// 使用空格将标注和引用类型分开

// &i32 //一个引用
// & 'ai32 带有显示生命周期的引用
// & 'a mut i32 带有显示生命周期的可变引用

// 泛型生命周期参数声明在： 函数名和参数列表之间的<>里 如下

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 泛型生命周期'a 得到的生命周期范围就是 x 和 y重叠的部分 取短的那一个

fn main() {
    let string1 = String::from("abcd");
    let res;

    {
        let string2 = String::from("xyz");

        res = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", res);
}

// error[E0597]: `string2` does not live long enough
//    --> life_cycle\src\main.rs:110:37
//     |
// 110 |     res = longest(string1.as_str(), string2.as_str());
//     |                                     ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
// 111 |
// 112 |   }
//     |   - `string2` dropped here while still borrowed
// 113 |   println!("The longest string is {}", res);
//     |                                        --- borrow later used here

// For more information about this error, try `rustc --explain E0597`.
// loongest函数返回的结果的生命周期是这两个字符串中比较短的那一个，也就是string2  而string2的生命周期范围在大括号内所以
// res 在大括号外 ，当它想访问string2的时候已经被销毁了所以报错
