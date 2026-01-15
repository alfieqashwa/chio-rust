#[derive(Debug, PartialEq)]
enum Gender {
  MALE,
  FEMALE,
}
#[derive(Debug)]
struct Children {
  name: String,
  age: u32,
  gender: Gender,
}

fn main() {
  let first_child = Children {
    name: "Qashwa Zhafira".to_string(),
    age: 14,
    gender: Gender::FEMALE,
  };

  let second_child = Children {
    name: "Cello El Zhafran".to_string(),
    age: 8,
    gender: Gender::MALE,
  };

  let third_child = Children {
    name: String::from("Chio El Zhafi"),
    age: 2,
    gender: Gender::MALE,
  };

  let initial_data = [first_child, second_child, third_child];

  // println!("Initial Data: {:#?}", initial_data);

  // for mut child in initial_data {
  //   child.age += 5;
  //   println!(
  //     "Name: {}, Age: {}, Gender: {:#?}",
  //     child.name, child.age, child.gender
  //   )
  // }

  let total_age: u32 = initial_data.iter().fold(0, |acc, child| acc + child.age);
  println!("Total age: {}", total_age);

  let update_data: Vec<Children> = initial_data
    .into_iter()
    .map(|mut child| {
      child.age += 5;
      child
    })
    .collect();

  let filtered_data: Vec<Children> = update_data
    .into_iter()
    .filter(|child| child.gender == Gender::FEMALE)
    .collect();

  for Children { name, age, gender } in filtered_data {
    println!("Name: {}, Age: {}, Gender: {:?}", name, age, gender);
  }
}
