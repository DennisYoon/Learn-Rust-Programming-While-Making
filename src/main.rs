use image;

fn main() {
  let white = image::Rgb::<u8>([255, 255, 255]); // 하얀색
  let red = image::Rgb::<u8>([255, 0, 0]); // 빨간색
  
  let w = 64; // 한 칸의 크기 (단위: pixel)
  
  let draw = |x, y| {
    let (xi, yi) = (x / w, y / w);
    match (xi % 2, yi % 2) {
      (0, 0) => white,
      (1, 0) => red,
      (0, 1) => red,
      (1, 1) => white,
      (_, _) => panic!("error")
    }
  };

  let img = image::ImageBuffer::from_fn(512, 512, draw);

  img.save("checkerboard.png").unwrap(); // 이미지 저장
}