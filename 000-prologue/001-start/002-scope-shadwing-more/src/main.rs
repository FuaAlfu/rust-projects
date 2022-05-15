fn main() {
    /*
      including broken code for
      testing/debuging purpose..
    */
    let x:i32 = 459;
    let mut y:i32 = 2323;
    let g: i32 = 99;
    {
        let xx:i32 = x * 2;
        let g = 97;
        println!("x: {} and xx: {}",x,xx);
        assert_eq!(g,88);
    }
    y = "here is my string";
    println!("{}",y);
    assert_eq!(g,234);

    let g = 76;
    println!("{}",g);
    my_func();
}

fn my_func(){
    let number:i32 = 32;
    println!("your number is: {}",number); 
}