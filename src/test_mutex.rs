use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::channel;
use std::thread;

const N: usize = 10;

pub fn test_mutex() {
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap()
            }
        });
    }

    println!("{:?}", data);

    rx.recv().unwrap();
}


pub fn test_rw_lock() {
    let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}

