/**
 * 当你把数据赋值给另外一个变量时，原有的变量将因失去数据的所有权而不能再访问变量，例如下面的代码当你试着运行时将会报错：
 */

use std::{collections::HashMap,fs::read_to_string};

// fn main(){
//   let source = read_to_string("./readme.md").unwrap();
//   let mut files = HashMap::new();
//   files.insert("这是我插入的一段话",source);
//   files.insert("这是我插入的第二段话",source);
// }

/**
 * 运行报错如下:
 * 
 *   ------ move occurs because `source` has type `String`, which does not implement the `Copy` trait
 *
 * 当我们将source插入 HashMap的时候 我嗯放弃了source的所有权，如果你想让上面的这段代码编译成，需要在第一次使用source时
 * 将其克隆
 * 
 * source.clone()
 */
fn main(){
  let source = read_to_string("./readme.md").unwrap();
  println!("{}",source);
  let mut files = HashMap::new();
  
  files.insert("这是我插入的一段话",source.clone());
  files.insert("这是我插入的第二段话",source);
}
