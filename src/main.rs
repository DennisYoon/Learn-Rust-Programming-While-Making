use rand::seq::SliceRandom;

fn main() {
  // 1부터 75의 값을 벡터에 대입
  let mut nums = Vec::new();
  for i in 1..=75 {
    nums.push(i);
  }

  // 섞기
  let mut rng = rand::thread_rng();
  nums.shuffle(&mut rng);

  // 카드표시
  for i in 0..25 {
    if i == 12 {
      print!("  *, ");
    } else {
      print!("{:3}, ", nums[i]);
    }
    
    if i % 5 == 4 {
      println!();
    }
  }
}