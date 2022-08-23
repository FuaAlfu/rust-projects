fn main() {
    struct MyStruct(i32);
    type a = i32;
    type b = (i32, i32);

    use MyStruct as UseAlias;

    let _ = UseAlias(87); 
    let example: a = 9221 as i32;
    let exampleTwo: (220, 110);
    println!("a is: {}",example);
}
