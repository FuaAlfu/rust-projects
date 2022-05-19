fn main() {
    //Tuples examples and more
    let (x,y,z) = (5,98,27);
    let (f,..) = (7,77,777); //ignoreing values after
    let (..,l)  = (81,80,79);//ignoreing values before
    let [..,n] = [1,2,4,6,8,10,12,14,15];

    println!("printing values of x , y, z {} {} {}",x,y,z);
    println!("printing values of f {}",f);
    println!("printing values of l {}",l);
    println!("printing values of n {}",n);
    assert_eq!([x,y,z,f,l,n],[5,98,27,7,79,15]);
    println!("success");
}