use std::borrow::BorrowMut;

/// 对于一个类型T的对象foo，如果T实现了AsRef<U>，
/// 那么foo可以通过调用.as_ref 得到一个类型为&U的新引用
fn is_hello<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref());
}

pub fn test_asref() {
    let s = "hello";
    is_hello(&s);
    println!("{}", s);

    let s = "hello".to_string();
    is_hello(&s);
    println!("{}", s);
}


fn add_one<T: AsMut<u64>>(num: &mut T) {
    *num.as_mut() += 1;
}

pub fn test_asmut() {
    let mut boxed_num = Box::new(0);
    add_one(&mut boxed_num);
    assert_eq!(*boxed_num, 1);
}

