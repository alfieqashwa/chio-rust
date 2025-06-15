#[derive(Debug)]
#[allow(dead_code)]
struct User {
  username: String,
  email: String,
  age: u32,
  is_active: bool,
}

fn build_user(username: String, email: String, age: u32) -> User {
  User {
    username,
    email,
    age,
    is_active: true,
  }
}

fn main() {
  let user1 = User {
    username: String::from("Alfie Qashwa"),
    email: String::from("alfieqashwa@gmail.com"),
    age: 42,
    is_active: true,
  };

  let update_user = build_user(
    "Qashwa Zhafira".to_owned(),
    "zhafira.qashwa@gmail.com".to_owned(),
    11,
  );
  println!("{:#?}", user1);

  println!("update user: {:#?}", update_user);
}
