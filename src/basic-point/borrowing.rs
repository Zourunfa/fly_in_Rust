/**
 * 引用不会获得值的所有权，只是借用值的所有权，引用数据的时候，如果数据是不可变的
 * 那么可以无数次借用，如果是可变的，则只能引用一次（主要出于对并发状态下发生数据访问
 * 碰撞考虑。我们在变量前面增加一个&便是这是一个引用，最常见的使用场景是不用克隆的情况下
 * 传递大量的数据）
 */


// use std::{collections::HashMap,fs::read_to_string};

// fn main(){
//   let source = read_to_string("readme.md").unwrap();
//   let mut files = HashMap::new();
//   files.insert("readme1",source.clone());
//   files.insert("readme2",source);

//   let files_ref =&files;
//   let files_ref2 = &files;

//   print_borrowed_map(files_ref);
//   print_borrowed_map(files_ref2);


// }



// fn print_borrowed_map(map: &HashMap<&str,String>){
//   println!("{:?}",map)
// }


// 如果你想创建一个可变的引用，需要将&改成&mut：

// use std::{collections::HashMap, fs::read_to_string};

// fn main() {
//     let source = read_to_string("./readme.md").unwrap();
//     let mut files = HashMap::new();
//     files.insert("readme", source.clone());
//     files.insert("readme2", source);

//     let files_ref = &mut files;
//     let files_ref2 = &mut files;

//     needs_mutable_ref(files_ref);
//     needs_mutable_ref(files_ref2);
// }

// fn needs_mutable_ref(map: &mut HashMap<&str, String>) {}

// 报错
// --> borrowing.rs:44:22
// |
// 43 |     let files_ref = &mut files;
// |                     ---------- first mutable borrow occurs here
// 44 |     let files_ref2 = &mut files;
// |                      ^^^^^^^^^^ second mutable borrow occurs here
// 45 |
// 46 |     needs_mutable_ref(files_ref);
// |                       --------- first borrow later used here

// 原因就是开头的那段话：如果是可变的，则只能引用一次，更通俗的理解就是你
// 将借用权给了file_ref2,所以不能使用它了


// 改变一下顺序才可以



use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let source = read_to_string("./readme.md").unwrap();
    let mut files = HashMap::new();
    files.insert("readme", source.clone());
    files.insert("readme2", source);

    let files_ref = &mut files;

    needs_mutable_ref(files_ref);

    let files_ref2 = &mut files;

    needs_mutable_ref(files_ref2);
}

fn needs_mutable_ref(map: &mut HashMap<&str, String>) {}

