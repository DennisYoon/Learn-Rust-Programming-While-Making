fn set(varaible: &mut i16, number: i16) {
  *varaible = number; // 역참조
}

fn main() {
  let mut variable = 1;
  set(&mut variable, 2);
  println!("{}", variable);
}