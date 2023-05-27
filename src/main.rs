fn add <T: std::ops::Add<Output = T> /* 덧셈 trait */> (a: T, b: T) -> T {
  return a + b;
}

fn x2<T: std::ops::Add<Output = T> + Copy /* 같은 매개변수 2번 사용해야 하므로 Copy trait도 지정 */> (n: T) -> T {
  return n + n;
}

fn x2verson2<T> (n:T) -> T
  where T: std::ops::Add<Output = T> + Copy {
    return n + n;
  }

#[derive(Debug)] // 구조체의 필드값을 println 매크로로 출력 가능
struct Point<T> {
  x: T,
  y: T
}

fn main() {
  println!("{}", add(1, 12)); // 13
  println!("{}", add(1.2, 5.8)); // 7.0
  println!("{}", add::<u8>(2, 0)); // 2 // 타입 명시
  println!("{}", add('a', 'b')); // char 타입은 Add trait을 overload하지 않음 -> 불가

  println!("{}", x2(2)); // 4
  println!("{}", x2verson2(2)); // 4

  let p: Point<f64> = Point {x: 2.0, y: 3.0};
  println!("x: {}, y: {}", p.x, p.y);
}