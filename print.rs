pub fn run() {
  println!("Hello from another file");

  println!("Binary: {:b}, Hex: {:x}, Demical: {:}",10,10,10);

  let mut age = 34;
  age = 23;
  println!("Age: {}", age);

  const ID: i32 = 001;

  println!("ID: {}",ID);
  
  let is_greater= 10>5;

  println!("{:?}", (is_greater));

  let mut hello = String::from("Hello ");

  println!("{:?}",(hello, hello.len(),hello);
}
