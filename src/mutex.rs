// https://kaisery.github.io/trpl-zh-cn/ch16-03-shared-state.html
#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_basic() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    #[test]
    fn test_multi_thread() {
        // Rc::new 单线程引用，以下例子无法使用
        // let counter = Rc::new(Mutex::new(0));

        // 多线程：原子引用计数 Arc<T>
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            // Rc::new 单线程引用，以下例子无法使用
            // let counter = Rc::clone(&counter);
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}