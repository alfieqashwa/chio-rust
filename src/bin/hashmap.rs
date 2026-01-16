use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  let blue = String::from("Blue");
  let yellow = String::from("Yellow");

  scores.insert(blue, 10);
  scores.insert(yellow, 10);

  scores.entry(String::from("Blue")).or_insert(50);
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Green")).or_insert(50); // added because there's no key Green

  scores.entry(String::from("Blue")).insert_entry(90); // update the value

  println!("{scores:#?}");

  let comparison = HashMap::from([
    (String::from("Blue"), 90),
    (String::from("Yellow"), 10),
    (String::from("Green"), 50),
  ]);

  assert_eq!(scores, comparison);

  let data = vec![("Qashwa", 1), ("Cello", 2), ("Chio", 3)];

  //   let mut children = HashMap::new();
  //   for (name, order) in data {
  //     children.insert(name, order);
  //   }
  //   OR

  let children: HashMap<&str, i32> = data.into_iter().collect();

  println!("{children:#?}");

  // Updating a Value Based on the Old Value
  let initial_kids = "chio qashwa cello chio cello chio";
  let mut kids = HashMap::new();

  for kid in initial_kids.split_whitespace() {
    let count = kids.entry(kid).or_insert(0);
    *count += 1;
  }

  println!("{kids:#?}");
}
