use std::fs::File;
use std::io;
use std::io:Read;

// 将Result枚举的两个类型 分别用具体的Sting 和Io Error代替
fn read_username_from_file() -> Result<String,io::Error>{
  let f = File::open("hello.txt");
  let mut f = match f{
    Ok(file) =>file,
    Err(e)=> return Err(e) //e就是具体发生的错误,
  };
  // 上面代码可以用下面一行代码替换
  // let mut f = File::open("hello.txt")?;


// >运算符：传播错误的一种快捷方式
// 如果Result是Ok： Ok中的值就是表达式的结果，然后执行程序
// 如果Result是Err Err就是整个函数的返回值 就像使用了return
  let mut s =String::new();
  // 最后一个表达式 就是函数的返回值可以不要分号
  match f.read_to_string(&mut s){
    Ok(_)=>Ok(s),
    Err(e)=>Err(e)
  }


  // Trait std::convert::From 上的From函数：
  // 用于错误之间的转换

  // 被？所应用的错误，会被隐式的from函数处理
  // 当问号调用from函数时
  // 它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
  // 常用于 针对不同的错误原因 返回同一种错误类型


}
// 下面代码 改进上面代码
// fn read_username_from_file() -> Result<String,io::Error>{
//   let mut s = String::new();
//   File::open("hello.txt")?.read_to_string(&mut s)?;
//   Ok(s)
// }

// ? 问号运算符只能是用于 Result枚举 Option枚举  或者实现了Try的类型

// main函数的返回类型 ()  单元类型  什么都没返回

// Box<dyn Error>是一个错误对象 可以简单理解为任何可能的错误类型
fn main()->Result<(),Box<dyn Error>> {
    // println!("Hello, world!");
    let result = File::open("hello.txt")?;
    Ok(())
}
