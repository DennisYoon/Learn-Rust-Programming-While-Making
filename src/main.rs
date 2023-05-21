use std::fs;

fn main() {
  // 현재 path는 src가 아님. ../src임
  let a = "src/a.txt";
  let b = "src/b.txt";

  // file 읽기
  let file_a = fs::read_to_string(a).unwrap();
  let file_b = fs::read_to_string(b).unwrap();

  // 불필요한 공백 삭제
  let file_a = file_a.trim();
  let file_b = file_b.trim();

  if file_a == file_b {
    println!("Same");
  } else {
    println!("Different");
  }
}