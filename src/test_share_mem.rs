use std::thread;
use std::sync::Arc;

static mut VAR: i32 = 5;

pub fn test_static() {
    let new_thread = thread::spawn(|| {
        unsafe {
            println!("static value in new thread {}", VAR);
            VAR = VAR + 1;
        }
    });

    new_thread.join().unwrap_or_default();
    unsafe {
        println!("static value in main thread : {}", VAR);
    }
}

pub fn test_box() {
    let var: Arc<i32> = Arc::new(5);
    let share_var = var.clone();

    let new_thread = thread::spawn(move || {
        println!("share value in new thread: {}, address: {}", share_var, &*share_var);
    });

    new_thread.join().unwrap_or_default();
    println!("share value in main thread: {}, address: {}", var, &*var);
}

