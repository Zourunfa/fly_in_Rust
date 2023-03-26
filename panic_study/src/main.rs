use core::{panic, panicking::panic};
use std::{fs::File, io::ErrorKind};

fn main() {
    // /**
    //  * Rust的错误处理
    //  * 大部分情况下 在编译时提示错误 并处理
    //  * 错误的分类：
    //  * 1.可恢复的错误，例如文件未找到，可再次尝试  Result<T,E>
    //  * 2.不可恢复的错误，bug，列如访问索引超出范围 panic!宏
    //  *
    //  *
    //  */
    // /**
    //  * 当panic红的执行：
    //  * 你的程序会打印一个错误信息
    //  * 展开 unwind 清理调用栈 stack
    //  * 退出程序
    //  *
    //  * 默认情况下，当panic发生：有下面两种操作
    //  * 程序会展开调用栈（工作量大）
    //  * Rust沿着调用栈往回走
    //  * 清理每个遇到的函数中的数据
    //  *
    //  * 或立即终止调用栈：
    //  * 不进行清理 直接停止程序
    //  * 内存需要OS进行清理
    //  *
    //  * 想让二进制文件更小 把设置从展开改为终止
    //  * 在cargo.toml 中的profile部分设置
    //  * panic="abort"
    //  */
    // panic!("crash and born")

    // let v = vec![1, 2, 3];
    // v[99];

    // panic!可能出现在自己的代码中，也可能出现我们依赖的代码
    // 通过对panic的调用定位问题

    // Result枚举
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // T 操作成功的情况下，OK变体里返回的数据类型
    // E 操作失败的情况下， Err变体里返回的错误的类型

    // 处理Result的一种方式 match表达式

    // let f = File::open("hellow.txt");

    // match f {
    //     Ok(file) => file,
    //     // 打开这个文件失败之后 对错误的类型再次match
    //     Err(error) => match error.kind() {
    //         // 尝试打开的文件不存在 如果不存在就创建 match 创建是否成功
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error create file"),
    //         },
    //         oe => panic!("Error  opening the file:{:?}", oe),
    //     },
    // };

    // unwrap: match表达式的一个快捷方法
    // 如果Result的结果是Ok 返回Ok里面的值
    // 如果Result的结果是Err 调用panic！宏

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Error opening file {:?}", error),
    // };

    // 可以使用下面代码替换上面的

    // let f = File::open("hello.txt").unwrap();
    // // 但是unwrap的错误信息是不能自定义的

    // // expect:和unwrap类似 但可以指定错误信息
    // let f = File::open("hello.txt").expect("无法打开错误文件");

    // 什么时候调用panic
    // 总体原则
    // 在定义一个可能失败的函数的时候 优先考虑返回Result
    // 否则使用panic

    // 1.有时候你比编译器掌握更多的信息,你可以确定Result就是OK，:unwrap

    // 你知道这个ip绝对是对的，绝对不会恐慌，你就可以使用unwrap
    let home::IpAddr = "127.0.0.1".parse().unwrap();

    // 2.当代码最终可能处于损坏状态的时候，最好使用panic
    //  损坏状态 ：某些假设 保证 阅读 或不可变性被打破
    // 非法的值或空缺的值被传入代码 不能预期发生

    // 场景建议调用你的代码传入无意义的参数值，panic
    // 调用外部不可靠代码 ，返回非法状态，无法修复：panic！
    // 如果失败是可预期的 最好是Result
    // 验证值的合法性 panic!
}

// oe就是othererror

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100,got {}", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

/*
这段代码定义了一个名为 Guess 的结构体，其中包含一个 i32 类型的字段 value。

结构体还定义了一个 new 函数，该函数接受一个 i32 类型的参数 value，并根据一些条件创建一个 Guess 结构体实例。如果传递的 value 值小于 1 或大于 100，则会出现 panic，打印出错信息并停止程序执行。否则，new 函数会使用传递的 value 值创建一个新的 Guess 实例，并返回该实例。

结构体还定义了一个名为 value 的方法，该方法返回存储在 Guess 实例中的 value 字段的值。value 方法为不可变引用(&self)方法，因此它不允许对 Guess 实例进行修改。 */
