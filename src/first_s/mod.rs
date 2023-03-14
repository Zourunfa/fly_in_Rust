// 除了应用之外 另一种不持有所有权的数据类型 ：切片

pub mod first_s {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); //返回字节 数组

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }
}

fn main() {
    let mut s = String::from("hello world");
    let worldIndex = first_word(&s);

    s.clear();
    println("{}", worldIndex)
}

// 字符串切边是指向字符串中一部分内容的引用

fn s() {
    let s = String::from("hello world");
    let hello = &s[0..5]; //&s[..5]
    let world = &[6..11]; //&[6..]       [..]全部字符
                          //
}

// 字符串字面值 就是切片,不可变的引用,
// 把某个变量借用为不可变的引用 就不能借用为可变的引用了

fn f() {
    let s = "hello";

    println("{}", s)
}
