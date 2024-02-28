// https://doc.rust-lang.org/std/ops/trait.Fn.html#calling-a-closure

// 闭包作为函数参数
fn call_with_one<F>(func: F) -> usize
    where F: Fn(usize) -> usize {
    func(1)
}


pub fn closure_example() {
    let square = |x| x * x;
    assert_eq!(square(5), 25);

    let double = |x| x * 2;
    assert_eq!(call_with_one(double), 2);
}

// https://course.rs/test/write-tests.html
#[cfg(test)]
mod tests {
    // 引入所有方法
    use super::*;

    #[test]
    fn closure_example_test() {
        closure_example()
    }
}