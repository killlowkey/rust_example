#[cfg(test)]
mod tests {
    #[test]
    fn test_box() {
        let a = Box::new(10);
        let b = *a + 10;
        assert_eq!(*a, 10);
        assert_eq!(b, 20);
    }

    #[test]
    fn test_mut_box() {
        let mut a = Box::new(20);
        println!("current value {}", a);
        *a += 10; // 修改值
        println!("current value {}", a);
    }


    // https://doc.rust-lang.org/stable/std/boxed/index.html
    #[test]
    fn test_box_move_stack() {
        // 原生类型实现了 copy，所以将 heap 数据移动到 stack 下面代码没问题
        let x = Box::new(10);
        println!("{}", x);
        let y = *x; // Move a value from a Box back to the stack by de-referencing:
        println!("{}", y);
        println!("{}", x);
    }

    #[test]
    fn test_box_move_stack2() {
        let x = Box::new(vec![1, 2, 3]);
        println!("{:?}", x);
        let y = *x;
        println!("{:?}", y);
        // 执行下面代码有问题，上面已经从 heap 移动数据到了 stack，下面直接 GG
        // 要是 vec 实现了 copy trait 就没啥问题
        // println!("{:?}", x);
    }
}