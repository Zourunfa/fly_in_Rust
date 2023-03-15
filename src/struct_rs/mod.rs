// struct：结构体

// 为相关联的值命名 打包成有意义的组合

// 定义
struct User {
    username: String,
    age: u64,
}

// 实例化

fn my_struct() {
    let u1 = User {
        username: String::from("abel"),
        age: 13,
    };

    // 一旦struc的实例是可变的，那么实例中所有的字段都是可变的

    // u1.age = 15

    // stuct的跟新语法
    let user2 = User {
        username: String::from("af"),
        ..u1
    };
}

// struct可以作为函数的返回值

fn build_User(name: String, age: u64) -> User {
    User {
        username: name,
        age, //可以简写
    }
}

// struct

// tuple struct
// derive就是派生的意思，可以引入很多trait为我们
// 的代码添加很多功能

#[derive(Debug)]
// rust里面包含打印信息调试的功能,
//  为自己的结构体显示的选择这一功能，需要在struct前面
// 加上这一注解
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", area(&rect));
    // 除开基础类型 都要实现一下 std::fmt::Display这个trait
    // 意思就是直接将类型以字符串的形式展示给用户

    // 或者使用 {:?}格式代替 但是要引入 #[derive(Debug)]

    // 这个Debug也是一种格式化的方法
    println!("{:?}", rect);
}
// &Rectangle 参数借用一下就行
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

// strcut 的方法
// 将上面的area方法定义到Rectangle上去
// 方法的第一个参数总是self,这个self就是结构体,
// 也可以获得其所有权或可变借用，和其他参数一样
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

// :: 符号用于关联函数
// 还可以用于模块创建的命名空间
fn main() {
    // 调用struct方法
    // 关联函数的调用
    let my_square = Rectangle::square(20);
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", rect.area());

    let rect1 = Rectangle {
        width: 50,
        length: 100,
    };

    println!("{}", rect1.can_hold(&rect));
    println!("{:?}", rect);
}

// 可以在impl里面定义关联函数，也就是不把self作为第一个参数的函数
// 例如 String::from 通常用于构造器
