use std::collections::HashMap;

fn main() {
  let data = "c,c,a,a,a,b,b,b,c,b,c,b,a,c,c,b,c,c,c";
  
  let mut map = HashMap::new();
  map.insert("a", 0);
  map.insert("b", 0);
  map.insert("c", 0);

  for char in data.split(",") {
    map.insert(char, map[char] + 1);
  }

  // 결과표시
  for k in "a,b,c".split(",") {
    println!("{}: {:>2}", k, map[k]);
    // map[..]이 없으면 패닉이 일어남. map.get(..)이용하면 해결
    // map.get()의 return값은 Option타입임
  }
}