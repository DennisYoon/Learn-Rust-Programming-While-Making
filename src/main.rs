struct Calculator {
  number: f64
}

impl Calculator {
  fn new(number: f64) -> Self {
    Calculator {
      number
    }
  } // constructor

  fn add(&mut self, extra: f64) {
    self.number += extra;
  }

  fn subtract(&mut self, extra: f64) {
    self.number -= extra;
  }

  fn divide(&mut self, extra: f64) {
    self.number /= extra;
  }

  fn multiply(&mut self, extra: f64) {
    self.number *= extra;
  }
}

fn main() {
  let mut calculator = Calculator::new(12.0);
  calculator.add(2.0); // 14.0
  calculator.divide(7.0); // 2.0
  calculator.subtract(9.0); // -7.0
  calculator.multiply(-1.0); // 7.0
  
  println!("{}", calculator.number);
}