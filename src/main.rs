use rand::Rng;

const MAP_N: usize = 25;
const TILES: [&str; 2] = ["⬜", "⬛"];

fn main() {
  let mut rng = rand::thread_rng();

  let mut maze = [[0; MAP_N]; MAP_N];

 // 둘레 벽으로 싸기
  for n in 0..MAP_N {
    maze[0][n] = 1;
    maze[n][0] = 1;
    maze[n][MAP_N - 1] = 1;
    maze[MAP_N - 1][n] = 1;
  }

  for y in 2..(MAP_N - 2) {
    for x in 2..(MAP_N - 2) {
      // 두칸마다 벽하나 설치
      if x % 2 == 1 || y % 2 == 1 {
        continue;
      }
      maze[y][x] = 1;

      // 상하좌우 중 하나를 벽으로 만들기
      let r = rng.gen_range(0..=3);
      match r {
        0 => maze[y - 1][x] = 1, // 상
        1 => maze[y + 1][x] = 1, // 하
        2 => maze[y][x - 1] = 1, // 좌
        3 => maze[y][x + 1] = 1, // 우
        _ => {}
      };
    }
  }

  // 미로 출력
  
  for y in 0..MAP_N {
    for x in 0..MAP_N {
      print!("{}", TILES[maze[y][x]]);
    }
    println!();
  }
}