fn main() {
  let banana = ("banana", 1000);
  let apple = ("apple", 500);

  println!("banana와 apple 가격 합: {}", banana.1 + apple.1);
  print_tuple(&banana);
  print_tuple(&apple);
}

fn print_tuple(item: &(&str, i32)) { // 튜플 자료형도 소유권 이동함
  println!("{}를 {}원에 구매", item.0, item.1);
}

// struct Item(&str, i32); 타입 선언하면 let banana = Item("banana", 1000); 같이 사용
// 배열도 소유권 이동하는 친구임