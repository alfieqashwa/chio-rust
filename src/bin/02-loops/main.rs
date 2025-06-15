mod for_in_loop;
mod loops;
mod while_loop;
fn main() {
  // loop
  loops::run();
  println!("===================");

  // while
  while_loop::run();
  println!("===================");

  // for
  for_in_loop::run();
}
