mod samefileload;

// path

/**
 * 路径的两种形式
 * 绝对路径：从crate root开始 使用crate 名或字面值
 * 相对路径; 从当前模块开始  使用self super或当前模块标志符
 *
 */

// pub mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 绝对路径调用
//     crate::front_of_house::hosting::add_to_waitlist();
//     // 相对路径调用
//     front_of_house::hosting::add_to_waitlist()
// }

// /**
//  * 1.模块不仅可以组织代码，还可以定义私有边界
//  * 2.如果想把函数或struct等设为私有，可以将它放到某个模块之中
//  * 3.Rust中所有的条目(函数，方法,struct,enum)默认都是私有的
// 4.如果加上pub关键字之后就是公共的
//  */
// 使用pub关键字 就会让mod 变为共有的

// super关键字：访问父级模块路径中的内容，类似文件系统中的..

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // 相对路径
//         super::serve_order();
//         // 绝对路径
//     }

//     fn cook_order() {}
// }

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn eat_at_restaurant() {
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like{} toast please", meal.toast);
        meal.seasonal_fruit = String::from("blueberries")
    }
}
