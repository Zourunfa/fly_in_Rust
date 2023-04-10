fn main() {
    // Vec<T> 叫做vector
    // 由标准库提供
    // 可存储多个值
    // 只能存储相同类型的数据
    // 值在内存中连续存放

    // 创建vec
    // let v: Vec<i32> = Vec::new();

    // 使用初始值
    // let mut v2 = vec![1,2,3];

    // // 添加元素
    // v2.push(1)

    //读取vec的元素 索引和get方式
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third "),
    // }

    // 所有权和借用规则
    // 不能在同一作用域内同时拥有可变和不可变引用

    // let mut v = vec![1,2,3,4,5];
    // // 声明一个不可变的借用
    // let first = &v[0];
    // vec的存放内存是连续的 使用push的时候可能会存在
    // 后面没地方了 这时候会全部重新分配一个地址
    // v.push(6);
    // println!("The first element is {}",first);

    // 遍历
    // let v = vec![100, 32, 57];
    // 使用不可变引用
    // for i in &v {
    //     println!("{}", i)
    // }

    // 使用可变引用修改
    // let mut v = vec![100, 32, 57];
    // for i in &mut v{
    //   // *i  * 是解引用符号
    //   *i+=50;
    //   println!("{}", i)
    // } 


    // 使用enum 来让vec里面具有多种类型的数据
      
    enum SpredsheetCell{
      Int(i32),
      Float(i64),
      Text(String)
    }

    fn main(){
      let row = vec![
        SpredsheetCell::Int(3),
        SpredsheetCell::Text(String::from("blue")),
        SpredsheetCell::Float(10.12)
      ];
      
    }
}
