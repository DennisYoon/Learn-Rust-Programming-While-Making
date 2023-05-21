use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  // 사전 파일 path 지정
  let dic_file = "src/dict.txt";

  // 명령줄 인수 vec에 저장
  let arg: Vec<String> = std::env::args().collect();

  // 인수 1개면 안됨
  if arg.len() < 2 {
    println!("인수 왜 한개만 줌??");
    return;
  }

  // 인수로 받은 단어
  let word = &arg[1];

  // 사전 file(dict.txt) 열기
  let fp = File::open(dic_file).unwrap();
  // BufReader로 읽어들임
  let reader = BufReader::new(fp);
  for line in reader.lines() {
    let line = line.unwrap();
    if line.find(word) == None {
      continue;
    }
    println!("{}", line);
  }

}