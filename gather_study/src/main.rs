use std::collections::HashMap;

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

    // enum SpredsheetCell{
    //   Int(i32),
    //   Float(i64),
    //   Text(String)
    // }

    // fn main(){
    //   let row = vec![
    //     SpredsheetCell::Int(3),
    //     SpredsheetCell::Text(String::from("blue")),
    //     SpredsheetCell::Float(10.12)
    //   ];

    // }

    // HashMap<K,V>
    // 泛型K,V(key,value)
    // 内部实现Hash函数:决定如何在内存中存放K和V

    // 创建空HashMap::new()
    // 通过Insert插入数据
    // use std::collections::HashMap;
    // let mut scores: HashMap<String,i32> = HashMap::new();
    // scores.insert(String::from("blue"),10);

    // HashMap 不在prelude预导入模块中
    // 标准库对其支持比较少，没有内置的宏来创建HashMap
    // 数据存储在heap上

    // HashMap是同构的：
    // 所有的K 必须是通一种类型
    // 所有的V 必须是同一种类型

    // 另一种创建HashMap的方式:collect方法

    // 在元素类型为Tuple的Vector上使用collect方法，
    // 可以组建一个HashMap
    // 要求Tuple有两个值：一个做为K，一个作为V
    // collect方法可以把数据整合成多种集合类型，包括HashMap

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let intial_scores = vec![10, 50];

    // // zip 有拉链的意思，两个vec 被缝合为元组数组
    // let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    // HashMap和所有权
    // 对于实现Copy trait的类型，例如i32，值会被复制到HashMap中
    // 对于拥有所有权的值，值会被移动，所有权会转移给HashMap

    // let field_name = String::from("Favorite color");
    // let field_value  = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name,field_value);
    // 从这一刻起 field_name field_value失效

    // 只有传值的引用的时候才不会失效
    // map.insert(&field_name,&field_value);

    // 在HashMap的有效期间，被引用的值必须保持有效

    // 访问HashMap中的值
    // get(K)  返回值Option枚举
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // match score {
    //     Some(s) => println!("{}", s),
    //     None => println!("team not exist"),
    // }

    // 遍历HashMap
    //     let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (k,v) in &scores{
    //   println!("{}: {}",k,v)
    // }

    // 更新HashMap
    // 每个key只能对应同一个value
    // 更新HashMap中的数据

    // k已经存在，对应一个V
    //  1.替换先要的V
    //  2.保留现有的V，忽略新的V
    //  3.合并现有的V和新的V

    // K不存在
    // 1.添加一对新的值

    let mut scores = HashMap::new();

    // 替换
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    
}
