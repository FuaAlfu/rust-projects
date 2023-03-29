fn main() {
    let x = 5;
    let y = if x > 0 { 1 } else { -1 };
    println!("Hello, world!");
}

fn pow(base: i32, exponent: u32) -> i32 {
    let mut result = 1;
    for _ in 0..exponent {
        result *= base;
    }
    result
}

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for &number in numbers {
        result += number;
    }
    result
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if a < b {
        None
    } else {
        Some(a / b)
    }
}