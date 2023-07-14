use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
  let companies = ["apple", "samsung", "google"];
  for company in companies {
    let message = format!("{}: {}", name, company);
    sender.send(message).unwrap();
    thread::sleep(Duration::from_millis(1000));
  }
  sender.send(String::from("done")).unwrap();
}

fn main() {
  // 스레드간 통신 채널
  let (sendr, receivr) = mpsc::channel::<String>();

  // 스레드 1 생성
  let sender = sendr.clone();
  thread::spawn(|| {
    sleep_sender("Team Cook", sender);
  });

  // 스레드 2 생성
  let sender = sendr.clone();
  thread::spawn(|| {
    sleep_sender("TM Roh", sender);
  });

  // 스레드로부터 지속적으로 메시지 받음
  loop {
    let buf = receivr.recv().unwrap(); // 메시지 받기
    println!("[수신] {}", buf);
    if buf == "done" {
      break;
    }
  }
}

/*
[수신] Team Cook: apple
[수신] TM Roh: apple
[수신] Team Cook: samsung
[수신] TM Roh: samsung
[수신] Team Cook: google
[수신] TM Roh: google
[수신] done
*/