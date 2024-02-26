// https://course.rs/basic/trait/generic.html
fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}

fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

pub fn old_add_example() {
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));
}

// std::ops::Add<Output=T> 特征 Trait 约束
// https://course.rs/basic/trait/trait.html
fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

pub fn add_example() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));
}
