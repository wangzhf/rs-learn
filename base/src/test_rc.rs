use std::rc::{Rc, Weak};

///
/// Rc使用引用计数的方法，让程序在同一时刻，实现同一资源的多个所有权拥有者，多个拥有者共享资源
/// 用于同一线程内部，通过 use std::rc::Rc 来引入。它有以下几个特点：
///
/// 1. 用 Rc 包装起来的类型对象，是 immutable 的，即 不可变的。即你无法修改 Rc<T> 中的 T 对象，只能读；
/// 2. 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
/// 3. Rc 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
/// 4. Rc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）。
pub fn test_rc() {
    let mut s = Rc::new("hello rust".to_string());
    println!("{:?}", s.chars());

    // 无法修改rc包装起来的对象
    // s.push('c');
}


///
/// Weak 通过 use std::rc::Weak 来引入。
///
/// Rc 是一个引用计数指针，而 Weak 是一个指针，但不增加引用计数，是 Rc 的 weak 版。它有以下几个特点：
/// 1. 可访问，但不拥有。不增加引用计数，因此，不会对资源回收管理造成影响；
/// 2. 可由 Rc<T> 调用 downgrade 方法而转换成 Weak<T>；
/// 3. Weak<T> 可以使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 值为 None；
/// 4. 常用于解决循环引用的问题。
pub fn test_weak() {
    let s = Rc::new("hello rust");

    let weak_s: Weak<&str> = Rc::downgrade(&s);
    println!("{:?}", weak_s);
    let strong_s: Option<Rc<&str>> = weak_s.upgrade();
    println!("{:?}", strong_s);
}
