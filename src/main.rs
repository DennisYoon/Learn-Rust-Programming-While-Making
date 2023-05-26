fn main() {
  /* 문자열에서 문자 한개씩 가져오기 */ {
    let s = "안녕하십니까?";
  
    // 첫 1글자 가져오기
    let ch1 = s.chars().nth(0).unwrap();
    println!("{}", ch1); // 안
  
    // 첫 1글자 가져오기
    let ch2 = &s[0..3];
    println!("{}", ch2); // 안
    // 한글은 한 글자가 3바이트임
  }
  
  /* &str과 String 상호 변환 */ {
    /* &str --> String */ {
      let s: &str = "Xin chào";
      let _ss1: String = String::from(s);
      let _ss2: String = s.to_string();
    }
    
    /* String --> &str */ {
      let s: String = String::from("Cảm ơn");
      let _ss1: &str = &s;
      let _ss2: &str = s.as_str();
    }
  }

  /* 문자열을 1바이트씩 출력 */ {
    let sentence = "시간은 금이다.";
    for c in sentence.bytes() {
      print!("{:2x} ", c);
    }

    println!("len: {}", sentence.len()); // len method는 길이가 아니라 byte 수를 반환

    let coll: Vec<char> = sentence.chars().collect();
    println!("글자수: {}", coll.len());
    println!("글자수: {}", sentence.chars().count());
  }

  /* 문자열을 1자씩 출력 */ {
    let sentence = "Do you hear the people sing??";
    
    let sentence_chars: Vec<char> = sentence.chars().collect();
    for c in sentence_chars.iter() /* 위의 code 때문에 sentence_char은 iterator가 아니다. iter()로 iterator로 다시 변환 */ {
      print!("({})", c);
    }
    println!();
  }

  /* 문자열 검색 */ {
    let s = "가야할 때가 언제인가를 분명히 알고 가는 이의 뒷모습은 얼마나 아름다운가";

    // 그냥
    match s.find("뒷모습") /* find는 문자열 탐색 method. Option을 반환한다. */ {
      Some(bi) => println!("뒷모습의 바이트 인덱스: {}", bi), // 64
      None => println!("뒷모습 없어!")
    };

    // 클로져 사용
    let res = s.find(|c:char| c == '뒷');
    match res {
      Some(bi) => println!("뒷의 바이트 인덱스: {}", bi), // 64
      None => println!("뒷모습 없어!")
    };
  }

  /* 문자열 치환 */ {
    let s: &str = "나무는 몸이 아팠다. 눈보라에 상처를 입은 곳이나...";
    let s: String = s.replace("나무", "바다"); // replace 메소드는 String은 반환.
    let s: String = s.replace("눈보라", "바닷 바람");
    println!("수정된 문자열: {}", s); // 바다는 몸이 아팠다. 바닷 바람에 상처를 입은 곳이나...
  }

  /* 문자열 분할 */ {
    let s: &str = "상처가 더 꽃이다.";
    
    // split_at으로 분할
    let (s1, s2) = s.split_at(9); // 특정 바이트 위치(여기선 9)에서 문자열 나눠서 (&str, &str) 튜플로 반환
    println!("{} / {}", s1, s2); // 상처가 / 더 꽃이다.

    let mut s1: String = String::from(s);
    let s2 = s1.split_off(9);
    // 위처럼 똑같이 문자열을 잘라서 s1에 첫번째 자른거, s2에 두번쨰 자른거 넣음. s1 값이 바뀌기에 s1에 mut 필수.
    println!("{} / {}", s1, s2); // 상처가 / 더 꽃이다.

    let s_div: Vec<&str> = s.split(" 더 ").collect(); // ' 더 '를 기준으로 문자열 나눠서 Vec<&str>에 넣음.
    println!("{} / {}", s_div[0], s_div[1]); // 상처가 / 꽃이다.
  }
}