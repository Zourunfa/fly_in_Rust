fn main() {
    // Rust的迭代器
    // 惰性特点： 除非调用消费迭代器的方法，否则迭代器本身没有任何效果

    // let v1 = vec![1, 2, 3];

    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    // Iterator Trait
    // 所有的迭代器都实现了Iterator trait
    // 实现Iterator trait 需要你定义一个Item类型，它用于next方法的返回类型

    // next方法
    // 每次返回迭代器中的一项
    // 返回结果包裹在Some里面
    // 迭代结束，返回None
}


#[cfg(test)]
mod tests{
  #[test]

  fn iterator_demo(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    // iter方法  在不可变引用上创建迭代器
    // into_iter()  创建迭代器会获得所有权
    // iter_mut  迭代可变的引用
    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
  }

}
