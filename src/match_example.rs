// https://course.rs/basic/match-pattern/all-patterns.html
pub fn match_example1() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

pub fn match_example2() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

pub fn match_example3() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

pub fn match_example4() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

pub fn match_example5() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Add;

    #[test]
    fn test_ownership_moved() {
        // 在 match 语句中使用 option、result，该所有权是否移动，取决于对值如何处理
        let name: Option<String> = Some("ray".into());
        match name {
            Some(n) => {
                println!("match statement value {n}"); // name's ownership moved
            }
            None => {
                println!("not found value");
            }
        }

        // 无法打印所有权已被移动
        // println!("outside statement value {}", name.unwrap())
    }


    #[test]
    fn test_ownership_not_moved() {
        // 在 match 语句中使用 option、result，该所有权是否移动，取决于对值如何处理
        let name: Option<String> = Some("ray".into());
        match name {
            Some(ref n) => {
                println!("match statement value {n}"); // 这里使用引用，那么所有权不会被移动
            }
            None => {
                println!("not found value");
            }
        }

        // 可以打印，因为在 Match 语句中使用的是引用
        println!("outside statement value {}", name.unwrap())
    }


    #[test]
    fn test_ownership_mut() {
        // 在 match 语句中使用 option、result，该所有权是否移动，取决于对值如何处理
        let mut name: Option<String> = Some("ray".into());
        match name {
            Some(ref mut n) => { // ref mut n: 表示的是 mut 的引用, 可以修改值
                println!("match statement value {n}"); // 这里使用引用，那么所有权不会被移动
                n.push_str(" hello");
            }
            None => {
                println!("not found value");
            }
        }

        // 可以打印，因为在 Match 语句中使用的是引用
        println!("outside statement value {}", name.unwrap())
    }
}