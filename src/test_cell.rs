use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

/// Cell只能用于T实现了Copy的情况
pub fn test_cell() {

    let c = Cell::new(5);
    let five = c.get();
    c.set(3);
    println!("{}", c.get());


    // 未实现Copy的话，需要使用mut
    let mut pc = Cell::new(Person { name: "jack".to_string(), age: 20, });
    let mut p = pc.get_mut();
    // pc.set(Person { name: "rose".to_string(), age: 25 });
    p.name = "rose".to_string();
    println!("{:?}" , p);
    println!("{:?}", pc.get_mut());

    // let c = Cell::new(String::from("hello rust"));
    // let s = c.get();


}


/// RefCell特点：
/// 1. 在不确定一个对象是否实现了 Copy 时，直接选 RefCell；
/// 2. 如果被包裹对象，同时被可变借用了两次，则会导致线程崩溃。所以需要用户自行判断；
/// 3. RefCell 只能用于线程内部，不能跨线程；
/// 4. RefCell 常常与 Rc 配合使用（都是单线程内部使用）；
pub fn test_ref_cell() {
    // 不可变引用
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    // 修改
    shared_map.borrow_mut().insert("africa", 88282);
    shared_map.borrow_mut().insert("marbles", 23433);
    println!("{:?}", shared_map);

    // 可同时存在多个不可变借用
    let rc = RefCell::new(String::from("hello"));
    let borrow1 = rc.borrow();
    let borrow2 = rc.borrow();

    // 多线程下使用会崩溃
    let result = thread::spawn(move || {
        let c = RefCell::new(5);
        let m = c.borrow_mut();
        // let b = c.borrow();  // 此处会panic
    }).join();

    // assert!(result.is_err());

}


struct Owner {
    name: String ,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

pub fn test_cell2() {
    // 创建一个可计数的Owner。
    // 注意我们将gadgets赋给了Owner。
    // 也就是在这个结构体里， gadget_owner包含gadets
    let gadget_owner: Rc<Owner> = Rc::new(
        Owner {
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        }
    );

    // 首先，我们创建两个gadget，他们分别持有 gadget_owner 的一个引用。
    let gadget1 = Rc::new(Gadget{id: 1, owner: gadget_owner.clone()});
    let gadget2 = Rc::new(Gadget {id: 2, owner: gadget_owner.clone()});

    // 我们将从gadget_owner的gadgets字段中持有其可变引用
    // 然后将两个gadget的Weak引用传给owner。
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

    // 遍历 gadget_owner的gadgets字段
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {

        // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
        // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
        // 断其所指向的对象是否存在。
        // 当然，这个Option为None的时候这个引用原对象就不存在了。
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);

        // 在main函数的最后， gadget_owner, gadget1和gadget2都被销毁。
        // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
        // 首先 gadget1和gadget2被销毁。
        // 然后因为gadget_owner的引用数量为0，所以这个对象可以被销毁了。
        // 循环引用问题也就避免了
    }
}
