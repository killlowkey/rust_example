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

// FnOnce 只执行一次，会获取 fn_once 参数的所有权
pub fn fn_once_example<T>(fn_once: T, str: String) -> usize
    where T: FnOnce(String) -> usize
{
    fn_once(str)
}

// FnMut只执行一次，会获取 fn_mut 参数的所有权
pub fn fn_mut_example<T>(mut fn_mut: T, str: String) -> usize
    where T: FnMut(String) -> usize
{
    // 这里会获取 str 的 mut 对象
    fn_mut(str)
}


// https://course.rs/advance/functional-programing/closure.html
struct Cacher<T, U: Clone>
    where
        T: Fn(U) -> U,
{
    query: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U>
    where
        T: Fn(U) -> U,
        U: Clone
{
    fn new(query: T) -> Cacher<T, U> {
        Cacher {
            query,
            value: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value.clone() {
            Some(v) => { v }
            None => {
                let res = (self.query)(arg);
                self.value = Some(res.clone());
                res
            }
        }
    }
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

    #[test]
    fn cacher_example() {
        let mut c = Cacher::new(|x| x);
        assert_eq!(c.value(1), 1);
        assert_eq!(c.value(100), 1);
        assert_eq!(c.value(109), 1);

        let mut c2 = Cacher::new(|x| x);
        assert_eq!(c2.value("hello"), "hello");
        assert_eq!(c2.value("hello1111"), "hello");
        assert_eq!(c2.value("222222"), "hello");
    }

    #[test]
    fn fn_once_example_test() {
        let fn_once = |s: String| { s.len() };
        let str = String::from("hello");
        // this parameter takes ownership of the value
        assert_eq!(fn_once_example(fn_once, str.clone()), 5);
        println!("{}", str);
    }

    #[test]
    fn fn_mut_example_test() {
        let fn_mut = |mut s: String| {
            s.push_str(" world");
            s.len()
        };
        let str1 = String::from("hello");
        // this parameter takes ownership of the value
        assert_eq!(fn_once_example(fn_mut, str1), 11);
    }
}