use rand::seq::SliceRandom;

fn main() {
  // 1부터 75까지 담긴 배열 선언
  let mut nums = [0; 75];
  for i in 1..=75 {
    nums[i - 1] = i;
  }

  // 섞기
  let mut rng = rand::thread_rng();
  nums.shuffle(&mut rng);

  // 카드 표시
  for y in 0..5 {
    for x in 0..5 {
      let i = y * 5 + x;
      if i == 12 {
        print!("  *, ");
      } else {
        print!("{:3}, ", nums[i]);
      }
    }
    println!();
  }
}