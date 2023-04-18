use std::thread;
use std::time::Duration;
fn main() {
    // 闭包：可以捕获其所在环境的匿名函数
    // 详细理解：1.匿名函数 2，保存为变量，作为参数
    //          3，可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
    //          4. 可以其定义的作用域捕获值

    // 生成自定义运动计划
    // 在必要的时候调用复杂算法 不必要的时候不调用 节省时间

    // 闭包的类型推断
    // 闭包不要钱标注参数和返回值类型
    // 闭包通常很短袖 只在狭小的上下文中工作 编译器通常能推断出类型

    // let a_closure = |x| x;

    // 没有下面的s 不知道类型
    // let s = a_closure(String::from("af"));  // x的类型在这里被确定了

    // 再使用必须是string类型

    // let simulated_user = 10;
    // let simulated_num = 7;
    // generate_workout(simulated_user, simulated_num)

    // // 闭包可以访问定义它的作用域内的变量，普通函数不能
    // let x = 4;
    // let equal_to_x = |z| z === x;
    // fn equal_to_x(z:i32)->bool{
    //   // 是函数的下面就报语法错误了 函数不能捕获他们的环境 会产生内存开销
    //   z==x
    // }

    // let   y =4;
    // assert!((equal_to_x(y)));

    // 闭包从所在环境捕获值的方式
    // 1.取得所有权: FnOnece
    // 2.可变借用: FnMut
    // 3.不可变借用:Fn

    // 创建闭包时，通过闭包对环境值的使用Rust推断出具体使用哪个Trait：
    // 所有的闭包都实现了FnOnce
    // 没有移动捕获变量的实现了FnMut
    // 无需可变访问捕获变量的闭包实现了FN

    // move关键字
    // 在参数列表前使用move关键字，可以强制闭包取得它使用的环境值的所有权
    // 当将闭包圈地给新线程以移动数据使其贵新线程所有时，此技术最有用

    let x = vec![1, 2, 3];
    // 使用move关键字的时候 x的所有权就移到了 闭包里面
    let equal_to_x = move |z: Vec<i32>| z == x;
    //  在上面这句话后面如果还想使用x 就报错
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println("calculating slowly");

//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups",
//             simulated_expensive_calculation(intensity)
//         );

//         println!(
//             "Next,do {} situps",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break tody! Remenber to stay hydrated");
//         } else {
//             println!(
//                 "Today, run for  {} mini",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }

// 优化1： 将函数的执行结果存储在变量中 函数只会被调用一次
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_res = simulated_expensive_calculation(intensity);
//     if intensity < 25 {
//         println!("Today, do {} pushups", expensive_res);

//         println!("Next,do {} situps", expensive_res);
//     } else {
//         if random_number == 3 {
//             println!("Take a break tody! Remenber to stay hydrated");
//         } else {
//             println!("Today, run for  {} mini", expensive_res);
//         }
//     }
// }

// 优化2： 如果是randow_number =3 的情况 那上面函数计算就是没必要的
// fn generate_workout(intensity: u32, random_number: u32) {
//   // 定义匿名函数 放在变量expensive_closure
//   let expensive_closure = |num| {
//     println!("calculating slowly ...");
//     thread::sleep(Duration::from_secs(2));
//   };

//   let expensive_res = simulated_expensive_calculation(intensity);
//   if intensity < 25 {
//       println!("Today, do {} pushups", expensive_closure(intensity));

//       println!("Next,do {} situps", expensive_closure(intensity));
//   } else {
//       if random_number == 3 {
//           println!("Take a break tody! Remenber to stay hydrated");
//       } else {
//           println!("Today, run for  {} mini", expensive_closure(intensity));
//       }
//   }
// }

// 优化3：创建一个结构体，它持有闭包及其调用结果
// 只会在需要结果的时候才执行该闭包
// 可缓存结果
//
// 这个模式通常叫做记忆化或延迟计算

// 如何让struct持有闭包
// struct的定义需要知道所有字段的类型
// 需要指明闭包的类型
// 每个闭包实例都有自己唯一匿名的类型，即使两个闭包签名完全一样

// 所以需要使用：泛型和Trait Bound

// Fn Trait
// 所有的闭包都至少实现了一下Trait之一
// Fn
// FnMut
// FnOnce

// struct Cache<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cache<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cache<T> {
//         Cache {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let mut expensive_res = Cache::new(|num| {
//         println!("caculation slowly ....");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });
//     if intensity < 25 {
//         println!("Today, do {} pushups", expensive_res.value(intensity));

//         println!("Next,do {} situps", expensive_res.value(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break tody! Remenber to stay hydrated");
//         } else {
//             println!("Today, run for  {} mini", expensive_res.value(intensity));
//         }
//     }
// }

// #[test]
// fn call_with_different_values() {
//     let mut c = Cache::new(|a| a);

//     let v1 = c.value(1);
//     let v2 = c.value(2);

//     assert_eq!(v2, 2);
// }
