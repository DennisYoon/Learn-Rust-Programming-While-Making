fn main() {
  let enc = encrypt("I LOVE YOU.", 3);
  let dec = encrypt(&enc, -3);

  println!("{} => {}", enc, dec);
}

fn encrypt(text: &str, shift: i16)-> String {
  let code_a = 'A' as i16; // 강제 형 변환
  let code_z = 'Z' as i16;

  let mut result = String::new();

  for ch in text.chars() {
    let mut code = ch as i16;
    if code_a <= code && code <= code_z {
      code = (code - code_a + shift + 26) % 26 + code_a;
    }

    result.push((code as u8) as char); // i16에서 char로 바로 바꾸기 불가
  }

  return result;
}