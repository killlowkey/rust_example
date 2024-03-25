#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_basic() {
        let handler = thread::spawn(|| {
            for i in 0..10 {
                println!("child thread current value {i}");
                thread::sleep(Duration::from_millis(3));
            }
        });

        for i in 0..10 {
            println!("master thread current value {i}");
            thread::sleep(Duration::from_millis(2));
        }

        // 等待 handler 线程执行完整
        handler.join().unwrap()
    }

    #[test]
    fn test_ownership() {
        let values = vec![1, 2, 3, 4];
        // 将 values 所有权移动到线程里面
        let handler = thread::spawn(move || {
            println!("values {:#?}", values)
        });

        // values 所有权已经丢失

        handler.join().unwrap()
    }

    #[test]
    fn test_channel() {
        // 线程a 与 线程b 通过 channel 来交互消息，类似于 golang channel
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            // tx 来发送消息
            tx.send(val).unwrap();
        });

        // rx 来接收消息
        println!("receive message from channel: {}", rx.recv().unwrap())
    }

    #[test]
    fn test_channel_loop_recv() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            // 无法发送引用，会发生所有权丢失错误
            // error example： tx.send(&val).unwrap();
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn test_channel_clone_recv() {
        let (tx, rx) = mpsc::channel();

        // 通过克隆发送者来创建多个生产者，后续 rx 可以接收多个生产者消息
        let tx1 = tx.clone();
        thread::spawn(move || { // 必须克隆，这里用到了 move 语义，tx1 所有权会转移，其它线程会无法使用
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    // https://doc.rust-lang.org/std/thread/struct.Thread.html
    #[test]
    fn test_thread_unpark() {
        let parked_thread = thread::Builder::new().spawn(|| {
            println!("Parking Thread");
            thread::park();
            println!("Thread Unparked");
        }).unwrap();

        thread::sleep(Duration::from_secs(2));
        println!("unpark the thread");

        parked_thread.thread().unpark();
        parked_thread.join().unwrap();
    }
}