use std::thread;
use std::time;
use std::sync::mpsc;
use std::fmt;
use std::rc::Rc;

/// 测试创建线程
pub fn test_thread() {

    test_create_thread();

    test_create_size_thread();
}

fn test_create_thread() {
    let new_thread = thread::spawn(|| {
        println!("I am a thread");
    });

    new_thread.join().unwrap_or_default();
}

fn test_create_size_thread() {
    // 创建一个线程，线程名称为thread1，堆栈大小为4k
    let new_thread_result = thread::Builder::new()
        .name("thread1".to_string())
        .stack_size(4 * 1024 * 1024)
        .spawn(|| {
            println!("I am a sized thread");
        });

    new_thread_result.unwrap().join().unwrap_or_default();
}


/// 测试消息传递
pub fn test_thread_msg_transmit() {

    // test_create_channel();

    test_create_channel2();

}

/// 测试发送i32类型数据
fn test_create_channel() {
    // 创建一个通道
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // 创建线程用于发送消息
    thread::spawn(move || {
        // 发送消息
        tx.send(1).unwrap();
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}

struct Student {
    id: u32,
}

impl fmt::Display for Student {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "student {}", self.id)
    }
}

fn test_create_channel2() {
    let (tx, rx): (mpsc::Sender<Rc<Student>>, mpsc::Receiver<Rc<Student>>) = mpsc::channel();

    thread::spawn(move || {
        // Rc未实现std::marker::Send, 所以此处无法传递
//        tx.send(Rc::new(Student {
//            id: 1,
//        })).unwrap();
    });

    println!("receive {}", rx.recv().unwrap());
}



/// 测试异步通道
///

const THREAD_COUNT: i32 = 2;

pub fn test_async_channel() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    for id in 0..THREAD_COUNT {
        // 注意Sender是可以clone的，这样就可以支持多个发送者
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id + 1).unwrap();
            println!("send {}", id + 1);
        });
    }

    let mill = time::Duration::from_millis(2000);
    thread::sleep(mill);
    println!("wake up");

    for _ in 0..THREAD_COUNT {
        println!("receive {}", rx.recv().unwrap());
    }
}

pub fn test_sync_channel() {
    /// 同步通道是需要指定缓存的消息个数的，但需要注意的是，最小可以是0，表示没有缓存。
    /// 发送者是会被阻塞的。当通道的缓存队列不能再缓存消息时，发送者发送消息时，就会被阻塞。
    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(0);

    let new_thread = thread::spawn(move || {
        println!("before send ");
        tx.send(1).unwrap();
        println!("after send ");
    });

    println!("before sleep");
    let mill = time::Duration::from_millis(5000);
    thread::sleep(mill);
    println!("after sleep");

    println!("receive {}", rx.recv().unwrap());
    new_thread.join().unwrap_or_default();
}
