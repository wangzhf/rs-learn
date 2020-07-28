use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn test_notify() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;

        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("notify main thread");
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
}

pub fn test_automic() {
    let var: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
    let share_var = var.clone();

    let new_thread = thread::spawn(move || {
        println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));

        share_var.store(9, Ordering::SeqCst);
    });

    new_thread.join().unwrap_or_default();
    println!("share value in main thread: {}", var.load(Ordering::SeqCst));
}


pub fn test_mutex() {
    let var: Arc<Mutex<u32>> = Arc::new(Mutex::new(5));
    let share_var = var.clone();

    let new_thread = thread::spawn(move || {
        let mut val = share_var.lock().unwrap();
        println!("share value in new thread: {}", *val);
        *val = 9;
    });

    new_thread.join().unwrap_or_default();
    println!("share value in main thread: {}", *(var.lock().unwrap()));
}


pub fn test_mutex_notify() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("notify main thread");
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
}
