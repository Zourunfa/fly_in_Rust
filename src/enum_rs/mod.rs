// 枚举 列举所有可能的值的类型

// 定义

enum IP_Area {
    V4, //变体
    V6,
}

struct IpAddr {
    kind: IP_Area, //可以作为结构体的一部分
    address: String,
}

// 枚举值

fn main() {
    let four = IP_Area::V4;
    let six = IP_Area::V6;

    route(four);
    route(six);

    route(IP_Area::V6);
}

fn route(ip_kind: IP_Area) {}

// 下面这种方式不需要使用额外的结构体
// 每个变体可以拥有不同的类型
enum IP_Area {
    V4(String), //变体
    V6(String),
}

enum Ip {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = Ip::V4((127), (0), (0), (1));
}

enum Message {
    Quit,                    //没有任何数据
    Move { x: i32, y: i32 }, //匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举定义方法
impl Message {
    fn call(&self) {}
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move {
        x: (12),
        y: (23 = 4),
    };
    let w = Message::Write((String::from("hello")));

    m.call()
}

// Option枚举
// 定义于标准库中
// 在prelude中 预先导入的模块 意味可以直接使用
// 描述了某个值可能存在或者不存在

// Rust没有null

// 但是Rust中类似Null概念的枚举 -》 Option<T>
enum Option<T> {
    Some(T),
    None,
}
// 可以直接使用Options<T>和它里面的两个变体

fn main() {
    let s = Some(5);
    let s_string = Some("Stringssss");
    // T就是泛型
    let absent_num: Option<i32> = None;
}

// 比null好的地方 Option<T>和T是不同的类型，不可以把
// Option<T>直接当成T
// 若想用Option中的T必须先将其转化为T

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x+y;
}
/**
 * 
error[E0277]: cannot add `std::option::Option` to `i8`
 */
