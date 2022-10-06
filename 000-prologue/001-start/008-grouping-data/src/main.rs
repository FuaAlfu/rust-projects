fn main() {
    let mut my_array: [i32; 8] = [876, 7, 3, 4,397,285,9,10];
    let empty_array: [u32; 0] = [];
    let mut names: [&str; 4] = ["john", "doe","sam","smith"];

    names[0] = "johny";

   // println!("all: {}", names);
  //  println!("First Element 0: {}", names[0]);
    println!("---");
  //  println!("all: {}", my_array);
    println!("all: {}", my_array[0]);

    //for testing
   // change_value(my_array);
   // rangeOverSlice(names)
    
}

// fn rangeOverSlice(mut s: &[&str] ){
//     for s in &s {
//     println!("element: {}", s);
// }
// }

// fn change_value(mut arr: &[i32]) {
//     arr[1] = 10;
//     println!("array after mod..{}",arr);
// }
