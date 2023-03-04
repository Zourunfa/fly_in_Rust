// ，一个字符串字面量"Hi!"不是String的实例

fn study_string(){
  print_type_of(&"H!");
  println(&String::new())
}

fn print_type_of<T>(_:&T){
  println!("Type is :{}",std::any::type_name::<T>())
}
