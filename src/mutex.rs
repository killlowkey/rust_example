// https://kaisery.github.io/trpl-zh-cn/ch16-03-shared-state.html
#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc, Mutex, RwLock};
    use std::thread;

    #[test]
    fn test_mutex() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap(); // lock 持有
            *num = 6;
        } // lock 释放

        println!("m = {:?}", m);
    }

    #[test]
    fn test_rw_lock() {
        let lock = Arc::new(RwLock::new(5));
        {
            let x = lock.read().unwrap(); // 离开作用域才会释放锁
            println!("x before write = {}", x);
        } // read lock 释放
        {
            let mut w = lock.write().unwrap();
            *w += 1;
            println!("Write value: {}", *w);
        } // 写锁释放
    }

    // 多线程 lock 需要使用 Arc 进行引用
    #[test]
    fn test_rw_lock2() {
        let mut handles = vec![];
        let lock = Arc::new(RwLock::new(5));

        for _ in 0..5 {
            let mut lock_clone = Arc::clone(&lock);
            let handle = thread::spawn(move || {
                {
                    let r1 = lock_clone.read().unwrap(); // Acquire a read lock
                    println!("Read value: {}", *r1); // Safe read without data race
                } // Read lock is automatically released here
                {
                    let mut w = lock_clone.write().unwrap(); // Acquire a write lock
                    *w += 1; // Modify the data safely
                    println!("Write value: {}", *w); // Safe write without data race
                } // Write lock is automatically released here
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Value after threads modified it: {}", *lock.read().unwrap());
    }


    #[test]
    fn test_multi_thread() {
        // Rc::new 单线程引用，以下例子无法使用
        // let counter = Rc::new(Mutex::new(0));

        // https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
        // T 为 heap 上的对象，使用 clone 克隆 Rc 或 Arc 对象后，会共享原先的 T 对象
        // 单线程：引用计数 Rc<T>
        // 多线程：原子引用计数 Arc<T>
        // Mutex::new(0) 在多线程中无法多次引用，需要使用计数器来实现
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            // Rc::new 单线程引用，以下例子无法使用
            // let counter = Rc::clone(&counter);
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap(); // 线程执行完、锁就会释放，锁跟随作用域
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    // https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
    #[test]
    fn test_channel() {
        // Create a new channel
        let (tx, rx) = mpsc::channel();
        // Spawn a new thread and move the sender into it
        thread::spawn(move || {
            // Send a message on the channel
            tx.send("Hello from the spawned thread!").unwrap();
        });
        // Receive the message in the main thread
        let received = rx.recv().unwrap();
        println!("Received: {}", received);
    }

    // https://doc.rust-lang.org/std/sync/mpsc/
    #[test]
    fn test_multithreading_channel() {
        // Create a new channel
        let (tx, rx) = mpsc::channel();
        for i in 0..5 {
            let sender = tx.clone();
            // Spawn a new thread and move the sender into it
            let handler = thread::spawn(move || {
                // Send a message on the channel
                sender.send(format!("Hello from the spawned thread! {}", i)).unwrap();
            });
        }

        for i in 0..5 {
            let received = rx.recv().unwrap();
            println!("Received: {}", received);
        }
    }
}