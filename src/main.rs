use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个原子引用计数的互斥锁，初始值为0
    let counter = Arc::new(Mutex::new(0));

    // 创建一个空的线程向量，用于存储线程句柄
    let mut handles = vec![];

    // 创建10个线程，每个线程将计数器增加1
    for _ in 0..10 {
        // 将原子引用计数的互斥锁克隆给每个线程
        let counter = Arc::clone(&counter);

        // 创建一个新线程
        let handle = thread::spawn(move || {
            // 获取互斥锁的所有权，如果其他线程已拥有，则等待
            let mut num = counter.lock().unwrap();

            // 增加计数器的值
            *num += 1;

            // 释放互斥锁
        });

        // 将线程句柄存储在向量中
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出计数器的值
    println!("Result: {}", *counter.lock().unwrap());
}
