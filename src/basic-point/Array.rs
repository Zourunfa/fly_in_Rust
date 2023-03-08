/**
 * rust数组相比于JavaScript数组的不足之处在于，
 * 长度必须固定且初始化的时候每个位置都要有初始值，如下面的代码就有问题：
 *
 */

// fn main() {
//     let mut numbers = [1, 2, 3, 4];
//     numbers.push(5);
//     println!("{:?}", numbers);
// }
/**
 *   |
9 |     numbers.push(5);
  |             ^^^^ method not found in `[{integer}; 4]`

error: aborting due to previous error

 */

/**
 *
 * 如果你希望得到得到类似JavaScript中的那种数组功能，
 * 你需要的是 Vec 或 VecDeque，Vec可以在尾部增减新内容，VecDeque则是首尾都可以
 *
 */
fn main() {
    let mut numbers = vec![1, 2, 3, 4];
    numbers.push(5);
    println!("{:?}", numbers);
}
