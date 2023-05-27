enum Currency {
  Currency100(usize),
  Currency500(usize),
  Currency1000(usize),
  Currency5000(usize),
  Currency10000(usize),
  Currency50000(usize),
}

impl Currency {
  fn calc_price(&self) -> usize {
    match *self {
      Currency::Currency100(v) => v * 100,
      Currency::Currency500(v) => v * 500,
      Currency::Currency1000(v) => v * 1000,
      Currency::Currency5000(v) => v * 5000,
      Currency::Currency10000(v) => v * 10000,
      Currency::Currency50000(v) => v * 50000
    }
  }
}

fn main() {
  let wallet = vec![
    Currency::Currency100(2),
    Currency::Currency1000(1),
    Currency::Currency5000(100)
  ];

  let total = wallet
    .iter()
    .fold(0, |sum, v| sum + v.calc_price()); // js의 reduce 클로져 쓴거랑 같음
  println!("지갑안 돈: {}원", total);
}