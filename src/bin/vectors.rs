fn main() {
  let mut animals = Vec::from(["cat", "dog", "bird", "ant"]);
  assert_eq!(animals.capacity(), 4);
  assert_eq!(animals.len(), 4);

  assert_eq!(animals[2], "bird");

  animals.push("cow");

  assert_eq!(animals, ["cat", "dog", "bird", "ant", "cow"]);
  assert_eq!(animals.len(), 5);
  assert_eq!(animals.capacity(), 8);

  assert_eq!(animals.pop(), Some("cow"));

  animals.extend(["spider", "bee", "mosquito"]);

  for animal in &animals {
    print!("{} ", animal)
  }
  assert_eq!(animals.len(), 7);

  assert_eq!(&animals[2..4], ["bird", "ant"]);
}
