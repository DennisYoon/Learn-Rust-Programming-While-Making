/* 소수를 반환하는 8비트 반복자 */
struct PrimeIterator {
  n: u8
}

/* 메서드 정의 */
impl PrimeIterator {
  fn new() -> Self {
    PrimeIterator { n: 1 }
  }

  // self.n이 소수인지 판별
  fn is_prime(&self) -> bool {
    for i in 2..self.n {
      if self.n % i == 0 {
        return false;
      }
    }
    return true;
  }
}

/* 반복자 구현 */
impl Iterator for PrimeIterator {
  type Item = u8; // 반환하는 type임 for x: Item in ~이 될 거임

  // 다음 소수 값을 반환
  fn next(&mut self) -> Option<Self::Item> {
    // 소수 찾아 반환
    loop {
      self.n += 1;
      // 8비트 넘으면 못찾음
      if std::u8::MAX == self.n {
        return None; // None 나올때까지 반복함
      }
      if self.is_prime() {
        return Some(self.n); // Some나오면 계속 next함
      }
    }
  }


}

fn main() {
  let prime_iter = PrimeIterator::new();

  /* 끝까지 돌리기 */
  for n in prime_iter {
    print!("{}, ", n);
  }
  println!();

  /* 몇개만 가져오기 */
  let prime_iter2 = PrimeIterator::new();
  let ten = prime_iter2.take(10);
  ten.for_each(|f| {
    print!("{}, ", f);
  })
  
}