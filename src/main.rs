use image::{self, imageops, GenericImageView};

fn main() {
  let size = 1280; // 리사이즈 후의 크기

  let mut img = image::open("windows.png").expect("파일 읽을 수 없음요"); // windows.png 파일 열기
  let dim: (u32, u32) = img.dimensions(); // 이미지 크기 얻기 (가로, 세로)형 튜플 반환
  
  // 정사각형으로 자르기
  let w = if dim.0 > dim.1 {dim.1} else {dim.0}; // 짧은 쪽을 길이로

  let mut img2 = imageops::crop(&mut img, (dim.0 - w) / 2, (dim.1 - w) / 2, w, w).to_image();
  // imageops::crop의 첫번째 매개변수: 자를 이미지, 두번째: 자르기 시작할 x 좌표, 세번째: y 좌표, 네번째: 가로 길이, 다섯번째: 세로 길이

  let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3);
  // imageops::resize의 첫번째 매개변수: 자를 이미지, 두번째: 리사이즈 되었을 때의 가로 pixel 수, 세번째: 세로 pixel 수, 네번째: 필터

  img3.save("windows2.png").unwrap();
}