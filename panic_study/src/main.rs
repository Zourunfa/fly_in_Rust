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

    let f = File::open("hello.txt").unwrap();
    // 但是unwrap的错误信息是不能自定义的

    // expect:和unwrap类似 但可以指定错误信息
    let f = File::open("hello.txt").expect("无法打开错误文件");
}

// oe就是othererror
