use rand::Rng;
use std::io::{self, Write};
// 将collections下面的所有公共mod引入
use std::collections::*;
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlis() {}
    }
}

// 使用use 引入的时候路径或名称默认是私有的
use create::front_of_house::hosting;

// 如果想要都可以访问需要加上pub,有点像js的export

pub fn eat_at_restatuarant() {
    // 前面可以不加hosting，
    // 但是 就不好区分add_to_waitlis方法是从哪里引入的了
    hosting::add_to_waitlis()
}

// 使用外部包
