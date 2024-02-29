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

pub fn option_method_example() {
    let v = vec![1, 2, 3, 4, 5, 30];

    // prints "got: 4"
    let x: Option<&usize> = v.get(3).inspect(|x| println!("got: {x}"));
    assert_eq!(x, Some(4).as_ref());
    // prints nothing
    let x: Option<&usize> = v.get(100).inspect(|x| println!("got: {x}"));
    assert_eq!(x, None);

    // assert_eq! 宏等价于如下代码
    // match (&x, &None) {
    //     (left_val, right_val) => {
    //         if !(*left_val == *right_val) {
    //             let kind = core::panicking::AssertKind::Eq;
    //             core::panicking::assert_failed(kind, &*left_val, &*right_val, None);
    //         }
    //     }
    // }

    let value = v.get(0).map(|x| x * x);
    assert_eq!(value, Some(1));

    assert_eq!(v.get(0).is_some(), true);
    assert_eq!(v.get(0).is_none(), false);
    assert_eq!(v.get(10).is_none(), true);

    fn is_gt_ten(v: &&usize) -> bool {
        *(*v) > 10
    }
    assert_eq!(v.get(0).filter(is_gt_ten), None);
    assert_eq!(v.get(5).filter(is_gt_ten), Some(30).as_ref());

    let value = v.get(100).map_or(99, |x| x*x);
    assert_eq!(value, 99);
    let value = v.get(0).map_or(99, |x| x*x);
    assert_eq!(value, 1);
}


