// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Armin";
  let mut age = 29;
  println!("My name is {} and I am {}", name, age);
  age = 28;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let ( my_name, my_age ) = ("Armin", 29);
  println!("{} is {}", my_name, my_age );
}
