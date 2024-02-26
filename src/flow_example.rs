pub fn for_example() {
    // 使用方法	                    等价使用方式	                                    所有权
    // for item in collection	    for item in IntoIterator::into_iter(collection)	转移所有权
    // for item in &collection	    for item in collection.iter()	                不可变借用
    // for item in &mut collection	for item in collection.iter_mut()	            可变借用

    // [1,10)
    for i in 1..10 {
        println!("{}", i);
    }

    // [1,10]
    for i in 1..=10 {
        println!("{}", i);
    }

    // continue
    for i in 0..=20 {
        if i % 10 != 0 {
            continue;
        }

        println!("{}", i)
    }

    // break
    for i in 0..10 {
        if i > 5 {
            break;
        }

        println!("{}", i)
    }
}

pub fn while_example() {
    let mut n = 0;
    while n < 5 {
        println!("{}", n);
        n += 1
    }
}

pub fn loop_example() {
    let mut x = 0;
    loop {
        if x == 5 {
            break;
        }

        x += 1;
    }
    println!("x={}", x);


    let y = loop {
        if x == 10 {
            break x;
        }
        x += 1;
    };
    println!("y={}", y)

}

pub fn if_example() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}