pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn option_example() {
    let five = Some(5);
    let six = plus_one(five);
    assert_eq!(Some(6), six);

    let none = plus_one(None);
    assert_eq!(None, none);
}
