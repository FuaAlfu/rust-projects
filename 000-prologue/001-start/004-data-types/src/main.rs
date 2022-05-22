/*
         signed    unsigned  variables
8 bit      i8      u8
16 bit     i16     u16
32 bit     i32     u32
64 bit     i64     u64
128 bit    i128    u128
arch       isize   usize
*/
fn main() {
    let a:i32 = 998;
    let f:f32;
    let ff:f64;
    let q:bool;
    let g = 5i32;
    let mut x = 69;

    let y = 5.0_f64;
    println!("success");

    f = 66.6;
    ff = 77.99;
    q = true;
    x = 9;

    println!("f: {}",f);
    println!("ff: {}",ff);
    println!("q: {}",q);
    println!("the value of a is {}, the value of g is {}",a,g);
    println!("the value of x is {}",x);
}
