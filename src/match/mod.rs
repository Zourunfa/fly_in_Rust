// 强大的控制流运算符 match
// 允许一个值和一系列的模式进行匹配，并执行匹配的模式对应的代码
// 模式可以是 字面值  变量名  通配符
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("31231");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 绑定值
        Coin::Quarter(state) => {
            println!("state from {:?}", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));
}

// Option<T>枚举
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 注意 ：match匹配的时候必须穷举所有可能

// 当需要匹配的值比较多的时候可以使用下划线通配符

fn main() {
    let v = 0u8;
    // 有 0 -255
    match v {
        1 => "one",
        3 => "three",
        _ => (), //什么也不会发生
    }
}

// if let 只关心一种匹配而忽略其他匹配

fn main() {
    let v = 0u8;
    // 有 0 -255
    match v {
        1 => "one",
        _ => "256", //什么也不会发生
    }

    // 可以用if let 替换
    if let Some(1) = v {
        "one"
    } else {
        "256"
    }

    // 搭配else
}
