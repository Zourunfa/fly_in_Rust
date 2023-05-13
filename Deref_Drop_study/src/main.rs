use std::ops::Deref;

/**
 * AI:
这是一个泛型结构体定义，名称为 MyBox，它包含一个参数 T。
结构体的定义方式为 struct MyBox<T>，其中 <T> 表示 T 为一个类型参数，可以是任何数据类型。
而 MyBox(T) 表示该结构体有一个成员变量 T，类型为 T，即与结构体同名的参数。
 */

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; //定义deref的关联类型
    fn deref(&self) -> &T {
        &self.0 //返回元组的第一个元素
    }
}

fn main() {
    // 实现Deref Trait使我们可以自定义解引用运算符*的行为
    // 通过实现Deref。智能指针可像常规引用一样来处理

    // 解引用运算符

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y); //出错 没有实现`{integer} == &{integer}`

    let z = Box::new(x);
    assert_eq!(5, *z);

    // 定义自己的智能指针
    // Box<T>被定义成拥有一个元素的tuple struct
    // 标准库中的Deref Trait 要求我们实现一个deref方法

    let r = MyBox::new(x);
    assert_eq!(5, *r); //*r =*(r.deref())

    // 如果没有deref trait * 只能对 &x常规解引用 ，有之后就可以
    // 对智能指针进行解引用
}
