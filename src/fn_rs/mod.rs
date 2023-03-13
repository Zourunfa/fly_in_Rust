// 将值传递给函数和把值赋给变量是类似的
// 将值传递给函数将发生移动或复制

fn main() {
    let s = String::from("Hello World");

    //s值 将所有权传递到函数内部
    // s将不再有效
    take_ownership(s);

    let x = 5; //i32

    // x因为它是i32类型，这个类型实现过copy的trait
    // 往函数传递的实际是x的副本
    makes_copy(x);

    println("x:{}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println("{}", some_number);
}

// 返回值和作用域
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    //s2已经在takes_and_givesback函数作用域了 所以不会被销毁
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // 把some_string移动给上面的S1
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


/**
 * 
 * 一个变量的所有权总是遵循下面模式：
 * 
 * 把一个值赋给其他变量时就会发生移动
 * 当一个包含heap数据的变量离开作用域时，它的值就会被drop
 * 函数清理，除非数据的所有权移动到另一个变量上
 */


// 如何让函数使用某个值但是不获得所有权



fn main(){
  let s1 = String:from("hello");
// s1作为参数传入  作为结果返回
  let (s2,len) = calculate_length(s1);

  println("The length of '{}' is {}.",s2,len)
}


fn calculate_length(s:String)->(String,usize){
  let length = s.len();
  (s,length)
}
