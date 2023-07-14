use std::{thread, time};

fn sleep_print(word: &str) {
  for i in 1..=3 {
    println!("{}: i={}", word, i);
    thread::sleep(time::Duration::from_millis(1000));
    // 1000ms 동안 프로그램 올 스톱
  }
}

fn main() {
  println!("---스레드 없을 때---");
  sleep_print("스레드 없음");

  println!("---스레드 이용---");

  // thread1
  thread::spawn(|| {
    sleep_print("운동에너지");
  });
  
  // thread2
  thread::spawn(|| {
    sleep_print("위치에너지");
  });

  // thread3
  thread::spawn(|| {
    sleep_print("역학적에너지");
  });

  // 메인 thread
  sleep_print("물리 과학")
}

/*
---스레드 없을 때---
스레드 없음: i=1
스레드 없음: i=2
스레드 없음: i=3
---스레드 이용---
물리 과학: i=1
운동에너지: i=1
위치에너지: i=1
역학적에너지: i=1
운동에너지: i=2
역학적에너지: i=2
위치에너지: i=2
물리 과학: i=2
물리 과학: i=3
역학적에너지: i=3
운동에너지: i=3
위치에너지: i=3
*/