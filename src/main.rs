fn print2(mut value: i32) {
  value *= 2;
  println!("{}", value);
}

fn main() {
  let value = 2;
  print2(value);
}