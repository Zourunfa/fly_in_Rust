// use demo::NewsArticel;
// use demo::Summary;
// 比较大小的Trait PartialOrd
// Copy Trait 解决 下面问题
// for &item in list.iter() {
//   |          ----    ^^^^^^^^^^^
//   |          |
//   |          data moved here
//   |          move occurs because `item` has type `T`, which does not implement the `Copy` trait 
//   |
// String 是存储在heap（堆上面的） 没有实现copyTrait

// 下面还是有不理解 ：
fn largest<T: PartialOrd +Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

fn main() {
    //
    // let article = NewsArticel {
    //     headline: String::from("dajisdjaisdojao"),
    //     author: String::from("sadasd"),
    //     locatin: String::from("aasd"),
    // };
    // println!("1 new tweet:{}", article.summarize())

    let number_list = vec![1, 2, 3, 4];
    let res = largest(&number_list);
    println!("The largest number is {}", res);
}
