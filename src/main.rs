fn main() {
  let mut stack: Vec<f64> = vec![]; // 스택 선언

  println!("RPN 입력 ㄱㄱ>>");

  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap(); // 식 입력 받기

  let ns = s.clone();

  let tokens = s.split_whitespace(); // 공백을 기준으로 split한 후 반복자 반환
  for token in tokens {
    let t = token.trim();

    // 숫자면 stack에 push
    match t.parse::<f64>() {
      Ok(v) => {
        stack.push(v);
        continue;
      }
      Err(_) => (),
    }

    // 연산자면 2번 pop하고 계산한 결과를 push
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();

    match t {
      "+" => stack.push(a + b),
      "-" => stack.push(a - b),
      "*" => stack.push(a * b),
      "/" => stack.push(a / b),
      _ => panic!("계산 불가 연산자: {}", t)
    }
  }

  // 결과표시
  println!("{} -> {}", ns.trim(), stack.pop().unwrap());
}