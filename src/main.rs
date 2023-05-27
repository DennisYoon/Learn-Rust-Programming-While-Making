trait TreasureBox {
  fn open(&self, key_no: i32) -> bool;
  fn check(&self);
}

struct JewelryBox {
  price: i32,
  key_no: i32
}

impl TreasureBox for JewelryBox {
  fn open(&self, key_no: i32) -> bool {
    self.key_no == key_no
  }
  fn check(&self) {
    println!("보석 상자임. {} 골드 입수함.", self.price);
  }
}

struct TrapBox {
  damage: i32
}

impl TreasureBox for TrapBox {
  fn open(&self, _key_no: i32) -> bool {
    return true;
  }
  fn check(&self) {
    println!("함정임. HP가 {}감소함.", self.damage);
  }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) { // TreasureBox trait으로 구현된 모든 structure을 타입으로 사용
  if !tbox.open(key_no) {
    println!("열쇠 안 맞음");
    return;
  }
  tbox.check();
}

fn main() {
  let box1 = JewelryBox {
    price: 30,
    key_no: 1,
  };

  let box2 = TrapBox {
    damage: 3
  };

  let box3 = JewelryBox {
    price: 20,
    key_no: 2
  };

  let my_key = 2;
  open_box(&box1, my_key);
  open_box(&box2, my_key);
  open_box(&box3, my_key);
}