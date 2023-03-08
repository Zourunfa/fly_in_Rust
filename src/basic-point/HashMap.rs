/**
 * 为了存储key 值数据，我们需要HashMap，虽然这不是唯一的选择，不过现在我们还不必考虑那么多
 */
use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert("key1", "value1");
//     map.insert("key2", "value2");

//     println!("{:?}", map.get("key1"));
//     println!("{:?}", map.get("key2"))
// }

/**
 * Some("value1")
   Some("value2")
 */

/**
 *some()：some()实际上是 Option enum 的一个变体，Option是另一种表达“无”的方式
 */

/**
 * 注意：我们可以用.is_some()或.is_none()来检测一个Option；
 * 可以通过.unwrap()来获取Some值，不过如果值是None的话代码会出问题，
 * 所以我们用.unwrap_or(default_value)代替。更多信息可见 Option

、
 */
fn main() {
  let mut map = HashMap::new();
  map.insert("key1", "value1");
  map.insert("key2", "value2");

  println!("{}", map.get("key1").unwrap_or(&""));
  println!("{}", map.get("key2").unwrap_or(&""));
}

/**
 * 
 * 为什么unwrap_pr的参数是&”“？
 * 1，HashMap的key和value 都是字符串字面量，所以rust自动推测类型 HashMap<&str, &str>
 * 2.然后get的返回值 必须是一个引用 如果不是引用 那么意味着HashMap将放弃对键值对的所有权 所以 &str的引用自然就是&&str
 * 3.upwrap_or的返回值 和get相同
 */
