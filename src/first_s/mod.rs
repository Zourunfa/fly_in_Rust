// 除了应用之外 另一种不持有所有权的数据类型 ：切片

pub mod first_s {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
    }
}
