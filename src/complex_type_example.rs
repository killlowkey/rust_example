// https://course.rs/basic/compound-type/tuple.html
pub fn string_example() {
    fn greetings(s: &str) {
        println!("{}", s)
    }

    fn get_str_len(s: &str) -> usize {
        s.len()
    }

    let a1 = "hello".to_string();
    println!("hello size = {}", get_str_len(&a1));

    let a2 = String::from("hello world");
    println!("hello world size = {}", get_str_len(a2.as_str()));

    let a3 = &a1[..1];
    greetings(a3);

    let a3 = &a1[2..3];
    greetings(a3);

    let a3 = &a1[2..];
    greetings(a3);
}

pub fn tuple_example() {
    let (x, y, z) = (1, "hello", 8.9);
    println!("x={} y={} z={}", x, y, z);

    // 原先的所有权 move 了
    fn fn_res(s: String) -> (String, usize) {
        let l = s.len();
        (s, l)
    }
    let name = "ray".to_string();
    println!("ray length = {}", fn_res(name).1);
    // 所有权已被移动，所以无法打印，这个例子只为举例，真实场景不使用
    // println!("{}", name)

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred={} six_point_four={} one={}", five_hundred, six_point_four, one);
}

enum PokerSuit<'a> {
    Clubs,
    Spades { x: i32, y: &'a str },
    Diamonds(&'a str),
    Hearts(u8, i32, &'a str),
}

pub fn enum_example() {
    let clubs = PokerSuit::Clubs;
    let spades = PokerSuit::Spades { x: 10, y: "spades" };
    let diamonds = PokerSuit::Diamonds("diamonds");
    let hearts = PokerSuit::Hearts(1, 10, "hearts");

    match hearts {
        PokerSuit::Clubs => println!("I am PokerSuit::Clubs"),
        PokerSuit::Spades { x: a, y: b } => println!("I am PokerSuit::Spades; x={} y={}", a, b),
        PokerSuit::Diamonds(name) => println!("I am PokerSuit::Diamonds; name={}", name),
        PokerSuit::Hearts(a, b, c) => println!("I am PokerSuit::Hearts; a={} b={}, c={}", a, b, c),
    }

    let x = match diamonds {
        PokerSuit::Clubs => {
            println!("I am PokerSuit::Clubs");
            1
        }
        PokerSuit::Spades { x: a, y: b } => {
            println!("I am PokerSuit::Spades; x={} y={}", a, b);
            2
        }
        PokerSuit::Diamonds(name) => {
            println!("I am PokerSuit::Diamonds; name={}", name);
            3
        }
        PokerSuit::Hearts(a, b, c) => {
            println!("I am PokerSuit::Hearts; a={} b={}, c={}", a, b, c);
            4
        }
    };
    println!("{}", x);

    // 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match
    if let PokerSuit::Diamonds(name) = diamonds {
        println!("if let = {}", name)
    }
}

pub fn array_example() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let array = [String::from("rust is good!"); 8]; // 写法错误，会进行 copy( the trait `Copy` is not implemented for `String`)
    // println!("{:#?}", array);

    // let array = [String::from("rust is good!"),String::from("rust is good!"),String::from("rust is good!")]; 写法太臃肿
    // println!("{:#?}", array);
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

pub fn vec_example() {
    let mut names = vec!["ray", "linda"];
    names.push("other");
    println!("{:?}", names);
    for (index, value) in names.iter().enumerate() {
        println!("index={} value={}", index, value);
    }
    println!("{:?}", names);

    let mut ages: Vec<u8> = Vec::new();
    ages.push(1);
    ages.push(2);
    ages.push(3);
    println!("{:?}", ages);

    println!("{}", ages.get(2).unwrap());

    let v = ages.get(10);
    if let Some(value) = v {
        println!("get successfully: {}", value)
    }
}