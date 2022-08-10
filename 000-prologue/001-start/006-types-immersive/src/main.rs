struct Person {
    name: String,
    age: u8,
}

fn main() {

    //types
    let a:i64 = 999;
    let mut name = String::from("alfu");
    let name = String::from("fua");
    let age:u8 = 27;

    //constructing..
    let p1 = Person{name, age: 29,};

    println!("a: {}",a * 2);
    println!("---");
    println!("p1 name {}",p1.name);
}
