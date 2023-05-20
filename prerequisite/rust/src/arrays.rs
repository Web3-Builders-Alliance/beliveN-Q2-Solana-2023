/* Arrays - Fixed list where elements are the same data types
index starts 0
 */

use std::mem;

pub fn run() {
  /* let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; */
  let mut numbers: [i32; 4] = [1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // Get single value
  println!("Single Value: {}", numbers[0]);

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated , dont use std inside
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}