use std::collections::HashMap;

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

pub fn result_example() {
    let result = Ok(true);
    let result2 = Err("error message");

    let vec1 = vec![result, result2];
    for res in vec1.iter() {
        match res {
            Ok(value) => println!("success: {}", value),
            Err(value) => println!("err: {}", value),
        }
    }
}

fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

// const 泛型（Rust 1.51 版本引入的重要特性）
fn display_array_new<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

pub fn display_example() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [i32; 2] = [1, 2];
    display_array(&arr);
}

pub fn display_array_new_example() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array_new(arr);

    let arr: [i32; 2] = [1, 2];
    display_array_new(arr);
}


pub fn hashmap_example() {
    // _ 代表 ?
    let mut map: HashMap<&str, u8> = HashMap::new();
    assert_eq!(map.insert("ray", 10), None);
    assert_eq!(map.insert("run", 1), None);
    assert_eq!(map.is_empty(), false);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("ray"), Some(10).as_ref());

    let map1 = HashMap::from([
        ("a", 1),
        ("b", 2)
    ]);
    for (key, value) in map1.iter() {
        println!("key={} value={}", key, value)
    }
}
