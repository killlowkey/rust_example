// https://course.rs/basic/lifetime.html
// 生命周期

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 函数：以下必须要手动添加生命周期，不知道这些引用存活的对象有多长，不知道是 x 长还是 y 长
// 只能通过 'a 来表示这些引用对象存活时间很长
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 编译器会添加如下的生命周期
// fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    s
}

fn longest_example() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}


// 结构体中的字段使用引入类型，也需要使用生命周期
// 不推荐结构体中使用引用，直接使用对应的所有权对象：比如 String
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn important_excerpt_example() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

#[derive(Copy, Clone, Debug)]
struct Person {
    age: i32,
}

fn return_owner(a: &Person) -> Person {
    // 必须要实现 #[derive(Copy, Clone)]
    *a
}

pub fn return_owner_example() {
    let a1 = Person { age: 10 };
    let a2 = return_owner(&a1);
    println!("a1 = {:?}", a1);
    println!("a2 = {:?}", a2);
}