fn sf(s: &str) -> String {
  String::from(s)
}

/* 반복자: 집합 데이터 구조(배열, vec같은)에 반복 처리(for문 돌리기)를 할 수 있게 해주는 추상 표현 */

fn main() {
  let array = [
    sf("Apple"),
    sf("Banana"),
    sf("Mango")
  ];

  for item in array /* array의 소유권이 이동한다 */ {
    println!("{}", item);
  }
  // println!("len: {}", array.len()); <에러 발생!!>

  // for문 실행시 암묵적으로 .into_iter method가 실행(for item in array.into_iter)

  // into_iter: 값 반환 반복자 반환. "소유권 이동"
  // iter: 값 참조 반복자 반환. "소유권 이동 x"
  // iter_mut: 가변 값 참조 반복자 반환. "소유권 이동 x"


  /* 수정본 */
  let array = [
    sf("Apple"),
    sf("Banana"),
    sf("Mango")
  ];
  
  for item in array.iter() {
    println!("{}", item);
  }
  println!("len: {}", array.len());
}