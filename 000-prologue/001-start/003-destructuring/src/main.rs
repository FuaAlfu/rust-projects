fn main() {
    let (x,y,z) = (5,98,27);
    let (f,..) = (7,77,777); //ignoreing values after
    let (..,l)  = (81,80,79);//ignoreing values before
    
    println!("printing values of x , y, z {} {} {}",x,y,z);
    println!("printing values of f {}",f);
    println!("printing values of l {}",l);
}
