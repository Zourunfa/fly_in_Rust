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

    // 消耗迭代器的方法
    // 在标准库中 iterator trait有一些带默认实现的方法
    // 其中有一些方法会调用next方法
    //  实现iterator trait 时必须实现next方法的原因之一
    // 调用next的方法叫做消耗型适配器
    // 因为调用他们会把迭代器消耗尽
    // 例如sum方法就会耗尽迭代器
    // 通过反复调用next，遍历所有元素
    // 每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和
    // 在下面 iterator_sum 测试方法中

    // 产生其他迭代器的方法
    // 定义在iterattor trait上另外一些方法叫做迭代器适配器
    // 把当前的迭代器转换为不同种类的迭代器
    // 可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用的可读性较高
    // map 方法 接收一个闭包，闭包作用于每个元素
    // 产生一个新的迭代器
}
// filter方法：
// 接收一个闭包
// 这个闭包在遍历迭代器的每个元素时，返回bool类型
// 如果闭包返回true，当前元素将会包含在filter产生的迭代器中
// 如果闭包返回false

// 使用iterator trait 创建自定义迭代器
pub struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn calling_next_direactly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))  //像拉链一样将两个迭代器缝合组成元组
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_ne!(18, sum)
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, show_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|x| x.size == show_size).collect()
    }

    #[test]
    fn filter_bt_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("nike"),
            },
            Shoe {
                size: 13,
                style: String::from("anta"),
            },
            Shoe {
                size: 10,
                style: String::from("snile"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("nike"),
                },
                Shoe {
                    size: 10,
                    style: String::from("snile"),
                },
            ]
        )
    }

    #[test]

    fn iterator_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        // iter方法  在不可变引用上创建迭代器
        // into_iter()  创建迭代器会获得所有权
        // iter_mut  迭代可变的引用
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]

    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        // 产生一个新的迭代器，用collect()方法消耗
        // 下滑线表示让编译器推断类型
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
}
