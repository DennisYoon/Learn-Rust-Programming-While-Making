fn main() {
  let s = String::from("RustProgrammingLanguage");
  println!("{}", &s[0..2]); // 0번째부터 1번째까지의 slice
  // slice는 기본적으로 참조자이기 때문에 소유권 없음 -> 기존 변수 계속 사용가능
  // s[..]은 전체 슬라이스 값임
}