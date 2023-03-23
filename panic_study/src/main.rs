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

    let v = vec![1, 2, 3];
    v[99];

    // panic!可能出现在自己的代码中，也可能出现我们依赖的代码
    // 通过对panic的调用定位问题
}
