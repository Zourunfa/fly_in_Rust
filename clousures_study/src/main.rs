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

    let a_closure = |x| x;

    // 没有下面的s 不知道类型
    let s = a_closure(String::from("af"));  // x的类型在这里被确定了

    // 再使用必须是string类型  下面不行
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
