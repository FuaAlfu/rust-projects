fn main() {
    limitedLoop();
}

fn infiniteLoop(){
    loop {
        println!("Hello, world!");
    }
}

fn limitedLoop(){
    for i in 0..30 {
        println!("Count: {}", i);
    }
}