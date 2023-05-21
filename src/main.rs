fn main() {
  let g1 = String::from("wow world");
  let g2 = &g1;
  // &는 g1에 대한 '참조자'로써 g1의 값을 '빌린 것'이다. 고로 소유권이 이동하지 않는다.
  println!("{} {}", g1, g2);

  println!("{}", hello_world());
}

fn hello_world() -> &str {
  let string = String::from("hello_world");
  return &string;
  // 참조자를 반환하는데 함수 scope이 끝나면 주인 string이 파기 되기 때문에 참조자를 값으로 사용할 수 없음.
}