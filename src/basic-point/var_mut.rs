fn main() {
    let mut mutable = 1; //声明时没有mut就是不可变数据，如果再次更改编译时就会报错
    println!("{}",mutable);
    mutable = 3;
    println!("{}",mutable);
}
