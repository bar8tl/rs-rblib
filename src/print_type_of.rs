// print_type_of: Display the data type of one object
pub fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}
