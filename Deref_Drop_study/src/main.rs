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


    // 函数和方法的隐式解引用转化 （ Deref Coercion )

    // 隐式解引用转化是为函数和方法提供一种便携特性
    // 假设T实现了Deref Trait 
    //  Deref Coercion 可以把T的引用转化为T经过Deref操作后生成的引用
    // 当把某类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配
    // 这个时候Deref Coercion就会自动发生
    // 编译器会对deref进行一系列的调用 来把它转为所需的参数类型
    // 上面步骤在编译时完成没有额外的性能开销



    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &  &MyBox<String>
    // deref &String  //string的内部也实现了deref trait
    // deref &str

    hello("Rust");



    // 解引用与可变性
    // 可使用DerefMut trait 重载可变引用的* 运算符
    // 在类型和trait 在下列三种情况发生时  Rust会执行deref coercion:
    // 当T:Deref<Target=U> 允许&T转换为&U时
    
}


//  Deref Coercion 

fn hello(name:&str){
  println!("hello, {}",name);
}


