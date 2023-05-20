fn main() {
  let a = input("첫번째 숫자 넣어라");
  let b = input("두번째 숫자 넣어라");

  println!("{}와 {}를 더한 값은 {}다.", a, b, a + b);
}

fn input(prompt: &str) -> f64 {
  println!("{}", prompt);

  // 입력값 가져오기
  let mut string = String::new();
  std::io::stdin().read_line(&mut string).expect("입력에러");

  // 공백 제거후 숫자 값으로 변환
  return string.trim().parse::<f64>().expect("숫자 아님");
  // expect는 에러메시지 심을 수 있음
  // unwrap은 그냥 에러 내보냄

  /* 
    match "string".parse() {
      Ok(v) => v, // 성공시
      Err(_) => 0, // 실패시
    }
  */
}