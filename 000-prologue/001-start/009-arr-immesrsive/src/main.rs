fn main() {
    let mut my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}",my_array[0]);
    println!("array size is :{}",my_array.len());

    //range over array
    for i in 0..5 {
        println!("index is: {} & value is : {}",i,my_array[i]);
     }

     my_array[1] = 7;
    println!("{:?}",my_array);

     new_array(&mut my_array)
}

fn new_array(arr:&mut [i32;5]){
    for i in 0..5 {
       arr[i] = 0;
    }
    println!("new array {:?}",arr);
  }
