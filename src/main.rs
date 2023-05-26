struct Object {
  mass: f64, // 질량(kg)
  height: f64, // 높이(m)
  velocity: f64 // 속력(m/s)  
}

impl Object {
  // 위치에너지 계산
  fn potential_energy(&self) -> f64 {
    let pe = self.weight() * self.height;
    return pe;
  }

  // 운동에너지 계산
  fn kinetic_energy(&self) -> f64 {
    let ke = 0.5 * self.mass * self.velocity.powf(2.0);
    return ke;
  }

  // 무게 계산
  fn weight(&self) -> f64 {
    let w = 9.8 * self.mass;
    return w;
  }
}

fn main() {
  let apple = Object {
    mass: 100.0,
    height: 10.0,
    velocity: 20.0
  };

  println!("위치 에너지: {:.2}J", apple.potential_energy());
  println!("운동 에너지: {:.2}J", apple.kinetic_energy());
  println!("무게: {:.2}kgf", apple.weight());
}