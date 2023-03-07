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
// fn main() {
//     let apples = 6;
//     let message = if apples > 8 {
//         "apple nums more than 8"
//     } else if apples < 4 {
//         "apple less 4"
//     } else {
//         "apple nums 4-8"
//     };

//     println!("{}", message) // prints "A few apples"
// }

// runst没有JavaScript中的null和undefined,不过rust依然有无这个概念
fn main() {
    let message = if apples > 10 {
        "lots of apples";
    } else if apples > 4 {
        "a few apples"
    } else {
        "a litte"
    };
}

/**
 * 报错
 * `if` and `else` have incompatible types
 * error[E0308]: `if` and `else` have incompatible types
  --> crates/day-7/syntax/src/main.rs:13:12
   |
11 |        let message = if apples > 10 {
   |   ___________________-
12 |  |         "Lots of apples";
   |  |         -----------------
   |  |         |               |
   |  |         |               help: consider removing this semicolon
   |  |         expected because of this
13 |  |     } else if apples > 4 {
   |  |____________^
14 | ||         "A few apples"
15 | ||     } else {
16 | ||         "Not many apples at all"
17 | ||     };
   | ||     ^
   | ||_____|
   | |______`if` and `else` have incompatible types
   |        expected `()`, found `&str`

   输出的信息中会提示你“expected (), found &str.”，而且还会提示你移除分号，不过提示里的()是什么意思呢？()被称为是单位类型（unit type），本质上意味着“无”，一个表达式如果是分号结尾，那么就会返回一个“无”，也叫unit type。如果我们只在if {}部分增加了分号，则这部分的返回结果是“无”，编译器希望每个条件分支返回相同的类型，所以当我们删掉分号之后，各个条件分支的返回结果类型就是相同的了。
当你在报错信息中看到()，就意味着你应该在某处添加或删除分号

 *
 */

/**
 * 函数的隐式返回
 * 函数的最后一个执行代码会作为返回值，而不必添加return声明： 注意不加分号
 */

fn add_numbers(left: i64, right: i64) {
    left + right
}

// 等价于下面代码
fn add_number(left: i64, right: i64) -> i64 {
    return left + right;
}
