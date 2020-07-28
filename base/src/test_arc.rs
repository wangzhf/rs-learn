use std::sync::Arc;
use std::thread;
use std::rc::Rc;

/// Arc 是原子引用计数，是 Rc 的多线程版本。Arc 通过 std::sync::Arc 引入。
/// 它的特点：
/// 1. Arc 可跨线程传递，用于跨线程共享一个对象；
/// 2. 用 Arc 包裹起来的类型对象，对可变性没有要求；
/// 3. 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
/// 4. Arc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）；
/// 5. Arc 对于多线程的共享状态几乎是必须的（减少复制，提高性能）。
pub fn test_arc() {
    let numbers: Vec<_> = (0..10u32).collect();
    let shared_numbers = Arc::new(numbers);

    for i in 0..10 {
        let child_numbers = shared_numbers.clone();

        thread::spawn(move || {
            let local_numbers = &child_numbers[..];
            println!("{} : {:?}", i, local_numbers);
        });
    }
}


struct Owner {
    name: String,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

pub fn test_demo() {

    let gadget_owner: Rc<Owner> = Rc::new(
        Owner { name: String::from("Gadget Man") }
    );

    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() };
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };

    // drop之后仍然可以使用
    drop(gadget_owner);

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

}
