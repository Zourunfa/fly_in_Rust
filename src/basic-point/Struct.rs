// 在TypeScript中我们可以这么实现一个interface
// interface TrafficLight {
//   color: string;
// }

// 类似的，在在rust中我们可以这么做：
// struct TrafficLight {
//     color: String,
// }

// fn main() {
//     let light = TrafficLight {
//         color: "red".to_owned(),
//     };
// }

// 在TypeScript我们可以使用class来进一步进行丰富，给予初始值并添加方法：

// class TrafficLight {
//   color: string;

//   constructor() {
//     this.color = "red";
//   }
// }

// const light = new TrafficLight();

// 在rust中的等价代码是,需要借助impl：
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn new() -> Self {
//         Self {
//             color: "red".to_owned(),
//         }
//     }
// }

/**
 * 我们增加了一个新的公共函数new() 在这里Self是TrafficLight的引用，我们可以通过TrafficLight::new()的
 * 方式调用
 *
 */

//  fn main() {
//   let light = TrafficLight::new();
//   println!("{}", light);
//   println!("{:?}", light);
// }

/**
 * 个代码是起作用的，但是我们无法验证它。
 * 你可以试着按如下的代码去打印，但会发现无法编译：
 * 无论是{}还是{:?}要想正确运行，都需要我们额外实现一些特性：
 *
 */

/**占时没真正理解 */

impl std::fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffic light is {}", self.color)
    }
}

/**
 * trait可以有一个推导出的默认实现，这有助你精简代码。如果你的struct所有字段都支持Debug特性，
 * 那么你可以通过添加一行#[derive(Debug)]来自动获取该能力：
 */
