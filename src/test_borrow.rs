use std::borrow::{Borrow, BorrowMut};

/// 对于一个类型为 T 的值 foo，如果 T 实现了 Borrow<U>，那么，foo 可执行 .borrow() 操作，即 foo.borrow()。操作的结果，我们得到了一个类型为 &U 的新引用
fn check<T: Borrow<str>>(s: T ) {
    assert_eq!("hello", s.borrow());
}

pub fn test_borrow() {
    let s = "hello".to_string();
    check(s);

    let s = "hello";
    check(s);
}

/// BorrowMut：BorrowMut<T> 提供了一个方法 .borrow_mut()。它是 Borrow<T> 的可变（mutable）引用版本。
/// 对于一个类型为 T 的值 foo，如果 T 实现了 BorrowMut<U>，那么，foo 可执行 .borrow_mut() 操作，即 foo.borrow_mut()。
/// 操作的结果我们得到类型为 &mut U 的一个可变（mutable）引用。
fn check2<T: BorrowMut<[i32]>>(mut v: T) {
    assert_eq!(&mut [1, 2, 3], v.borrow_mut());
}

pub fn test_borrow_mut() {
    let v = vec![1, 2, 3];
    check2(v);
    // println!("{:?}", v);
}

/// ToOwned 为 Clone 的普适版本。它提供了 .to_owned() 方法，用于类型转换
pub fn test_owned() {
    let s = "a";
    let ss = s.to_owned();
    println!("{}", s);
    println!("{}", ss);

    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();
    println!("{:?}", v);
    println!("{:?}", vv);
}
