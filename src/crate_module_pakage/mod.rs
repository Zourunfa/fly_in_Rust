// 模块系统
// Package 包:Cargo的特性 让你构建 测试 共享crate
// Crate 单元宝： 一个模块树 它可以产生一个library可执行文件
// Module 模块 use ： 让你控制代码的组织 作用域 和私有路径
// Path(路径) 为struct function module 等项命名的方式

/*
// Crate的类型
binary
library


Crate Root
是源代码文件
Rust的编译器从这里开始 组成你的Crate根Module


一个Pakage
包含1个Cargo.toml 它描述了如何构建这些Crates
只能包含0-1个library crate
它可以有任意数量的binary crate
它必须至少包含一个crate (libray或binary)


Cargo的惯例
src/main.rs
-binary crate的crate root
-crate名 和package的名相同

src/lib.rs
package包含一个library crate
library crate的crate root


cargo 会把crate root 文件交给rustc 来构建library或binary


一个Package 可以同时包含src/main.rs 和src/lib.rs
一个binary crate 和一个library crate


一个Package可以有多个binary crate

文件放在src/bin  每个文件都是单独的binary crate


Crate的作用 将相关的功能组合到一个作用域内
便于在项目中共享
如果有一个Crate的 Rand
Rand crate 访问它的功能需要通过它的名字


定义module来控制作用域和私有性
Module是在一个Crate中 将代码进行分组
增加代码的可读性 易于复用
控制项目item的私有性 public private
 */

// 简历module
// mod 关键字
// 可嵌套
// 定义其他项  stuct enum 等

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
