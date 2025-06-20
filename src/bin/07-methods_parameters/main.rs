#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn width(&self) -> bool {
    self.width > 0
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  // Methods with more paramaters
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // Associated functions
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width())
  }

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  // print square method
  let sq = Rectangle::square(20);
  println!("Square {:#?}", sq);
}
