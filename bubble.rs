use std::io;

fn main(){
  let mut array: [i32; 5] = [5,6,2,9,1];
  let mut tmp :i32;
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("failed to read input.");
  let n = input.trim().parse::<i32>().expect("invalid input");
  println!("N: {:?}",n);
  println!("Before :{:?}",array);
  for i in 0..array.len(){
    for j in 0..(array.len()-i-1){
      if array[j] > array[j+1] {
        tmp = array[j];
        array[j] = array[j+1];
        array[j+1] = tmp;
      }
    }
  }
  println!("After :{:?}",array);
}
