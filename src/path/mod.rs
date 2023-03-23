// path

/**
 * 路径的两种形式
 * 绝对路径：从crate root开始 使用crate 名或字面值
 * 相对路径; 从当前模块开始  使用self super或当前模块标志符
 *
 */

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径调用
    front_of_house::hosting::add_to_waitlist()
}


