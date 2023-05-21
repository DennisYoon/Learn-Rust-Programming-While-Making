use std::fs::{File, self};
use std::io::{Write, BufWriter};

fn main() {
  // 출력할 파일이름
  let out = "src/out.txt";

  // 파일로 저장할 부분 scope 지정
  {
    // 파일 생성
    let fp = File::create(out).unwrap();
    let mut writer = BufWriter::new(fp);
    
    // 문자열 리지털을 byte로 저장
    let s = String::from("hello");
    let bytes = s.as_bytes();

    writer.write(bytes).unwrap();
    // 파일 저장~
  } // <- file은 여기서 자동으로 닫힘

  let s = fs::read_to_string(out).unwrap();
  println!("{}", s);
  // 저장된 파일 내용 출력
  // hello 나올 거임
}