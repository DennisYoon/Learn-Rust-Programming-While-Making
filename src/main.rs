fn main() {
  let g1 = String::from("wow world");
  let g2 = g1.clone(); // 원시 타입이 아닌 타입(String, Structure)은 clone해주면 복사시켜 소유권 이동 안함

  println!("{} {}", g1, g2);
}

// 원시 타입에게는 소유권 시스템이 적용되지 않음
// 원시 타입은 stack 메모리에 저장되기 때문에 값을 복사(copy)함

// 원시 타입 외의 타입에게는 소유권 시스템이 적용됨
// 이 타입들은 heap 메모리에 저장되어 값을 이동(move)시킴

// 단 원시 타입이 아니더라도 Copy trait으로 구현한 타입이면 데이터 복사함(원시 타입은 모두 Copy Trait)