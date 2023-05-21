fn x2(arg: &mut i32) { // &mut는 값을 바꿀 수 있는 참조자라는 뜻
  *arg = *arg * 2; // 역참조는 *로
}

fn main() {
  let mut v = 16;
  x2(&mut v);
  println!("{}", v);
}