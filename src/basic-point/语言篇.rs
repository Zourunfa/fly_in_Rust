/*

rust的编程风格和JavaScript有些许的不同。
在rust中函数、变量、模块的命名方式是下划线式的，如time_in_millis
；结构体则是驼峰式的，如CpuModel；常量则是大写的下划线方式，如GLOBAL_TIMEOUT
 */

/*
括号的可选性
下面的写法是比较常见的
 */
// if (x > y) { /*

//   */ }

// while (x > y) { /* */ }

// // 不过下面的方式在rust中才是首选，上面的写法linter会给出警告
// if  x > y {

// }

// while x > y {}

// 表达式返回值
/**
 * 几乎所有完整的代码段都有返回值，如4 * 2返回8，if true { 1 } else { 0 }返回1，
 * 这就意味着你可以将表达式给变量赋值或者作为函数的返回值
 */
fn main() {
    let apples = 6;
    let message = if apples > 8 {
        "apple nums more than 8"
    } else if apples < 4 {
        "apple less 4"
    } else {
        "apple nums 4-8"
    };

    println!("{}", message) // prints "A few apples"
}
