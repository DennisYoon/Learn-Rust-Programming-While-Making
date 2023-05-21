struct Person {
  name: String,
  age: u8,
  height: u8,
}

fn main() {
  let me = Person {
    name: String::from("Dennis"),
    age: 16,
    height: 180
  };

  print_my_info(&me);
}

fn print_my_info(info: &Person) {
  println!("이름: {}, 나이: {}, 키: {}", info.name, info.age, info.height);
}