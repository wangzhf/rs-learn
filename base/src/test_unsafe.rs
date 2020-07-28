
pub fn test_unsafe() {
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("raw {:?}", raw);
    println!("raw points at {}", points_at);
}

static mut N: i32 = 5;

pub fn test_rw_static() {
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}

// 调用unsafe函数
unsafe fn is_unsafe() {
    println!("I am unsafe");
}

pub fn test_invoke_unsafe() {
    unsafe {
        is_unsafe();
    }
}


// 测试原始指针
pub fn test_point() {
    // 创建裸指针
    let a = 1;
    let b = &a as *const i32;
    let mut x = 2;
    let y = &mut x as *mut i32;


    // 解引用
    let a = 1;
    let b = &a as *const i32;
    let c = unsafe { *b };
    println!("{}", c);

    // Box的into_raw
    let a: Box<i32> = Box::new(10);
    let b: *const i32 = &*a;
    let c: *const i32 = Box::into_raw(a);

    // 引用和裸指针之间可以隐式转换，但隐式转换后再解引用需要使用unsafe
    let a = 1;
    let b: *const i32 = &a as *const i32;
    // 隐式
    let c: *const i32 = &a;
    unsafe {
        println!("{}", *c);
    }
}
