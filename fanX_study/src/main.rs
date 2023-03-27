// 函数中定义泛型
// 只有实现std::cmp::PartialOrd trait 才能比较大小
// std::cmp::PartialOrd

// 函数泛型
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if (item > largest) {
            largest = item
        }
    }
    largest
}

// 结构体泛型
// 结构体可以使用多个类型参数
struct Point<T, U> {
    x: T,
    y: U,
}

// 枚举泛型
// 主要是用在变体中

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 方法中定义泛型
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
// 对比没有实现泛型的
impl Point<i32, u32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}


// 方法的类型参数也可以和结构体不同
fn main() {
    // println!("Hello, world!");
    // 泛型是具体类型或其他属性的抽象代替
    // 你编写的代码不是最终代码而是一种模板 占位符
    // 、

    let number_list = [34, 50, 25, 100, 65];
    let res = largest(&number_list);
    println!("The largest number is {}", res);

    let char_list = vec!['2', 'a', 'c'];
    let res = largest(&char_list);
    println!("The largest number is {}", res);

    let interger = Point { x: 5, y: 1.0 };
}
