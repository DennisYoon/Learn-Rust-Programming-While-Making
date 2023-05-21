fn main() {
  {
    let g1 = String::from("hello world");
    let g2 = g1; // String객체의 소유권이 g1에서 g2로 이동
    
    println!("{}", g1); // 소유권이 이동했기에 g1 사용 불가
    println!("{}", g2);
  }
  

  {
    {
      let g = String::from("hey world");
    } // g의 scope가 끝남 -> 파기
    println!("{}", g); // g가 파기되었기에 사용 불가
  }
}