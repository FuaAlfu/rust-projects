fn main() {
    let x:i32 = 459;
    {
        let xx:i32 = x * 2;
        println!("x: {} and xx: {}",x,xx);
    }
    my_func();
}

fn my_func(){
    let number:i32 = 32;
    println!("your number is: {}",number); 
}