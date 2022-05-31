fn main() {
    let mut _multable_integer = 7i32;
    {
        //freezing :: "not multi able anymore"
       let __multable_integer =  _multable_integer;
       println!("{}",_multable_integer);
    }

    _multable_integer = 1;
    println!("{}",_multable_integer);
}
