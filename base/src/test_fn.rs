
pub fn test_fn() {

    let x: i32 = diverges();
    let y: String = diverges();
    println!("x is {}", x);
    print!("y is {}", y);

}

// 返回类型`!`可以用作任何类型
fn diverges() -> ! {
    panic!("This function never returns!");
}

pub fn test_closure() {
    let num = 5;
    let plus_num = |x: i32| x + num;
    let v = plus_num(5);
    println!("v is {}", v);

    let mut step = 10;
    // 使用move将step的所有权交给闭包
    let mut multiply_num = move |x: i32| -> i32 {
        step *= x;
        step
    };
    let m = multiply_num(2);
    println!("m is {:?}", m);
    // i32类型实现了Copy trait，因此还可以使用
    println!("step is {}", step);

    // 不使用move
    let mut x: String = String::from("abc");
    {
        // 包体内会对x进行可变借用，而不是剥夺x的所有权，外部添加大括号是为了作用域结束后让可变借用失效
        let mut some_closure = |c: char| x.push(c);
        some_closure('d');
    }
    println!("x={:?}", x);
}


// 测试高阶函数，允许把闭包作为参数来生成新的函数

fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply<F>(f: F, y: i32) -> i32
    where F: Fn(i32) -> i32 {
    f(y) * y
}

fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}


pub fn test_high_order_fn() {
    let transform: fn(i32) -> i32 = add_one;
    let f0 = add_one(2i32) * 2;
    let f1 = apply(add_one, 2);
    let f2 = apply(transform, 2);
    println!("{}, {}, {}", f0, f1, f2);

    let closure = |x: i32| x + 1;
    let c0 = closure(2i32) * 2;
    let c1 = apply(closure, 2);
    let c2 = apply(|x| x + 1, 2);
    println!("{}, {}, {}", c0, c1, c2);

    let box_fn = factory(1i32);
    let b0 = box_fn(2i32) * 2;
    let b1 = (*box_fn)(2i32) * 2;
    let b2 = (&box_fn)(2i32) * 2;
    println!("{}, {}, {}", b0, b1, b2);

    let add_num = &(*box_fn);
    let translate: &dyn Fn(i32) -> i32 = add_num;
    let z0 = add_num(2i32) * 2;
    let z1 = apply(add_num, 2);
    let z2 = apply(translate, 2);
    println!("{}, {}, {}", z0, z1, z2);
}

///
/// Rust通过impl关键字在struct、enum或者trait对象上实现方法调用语法 (method call syntax)。
/// 关联函数 (associated function) 的第一个参数通常为self参数，有3种变体：
/// `self`，允许实现者移动和修改对象，对应的闭包特性为`FnOnce`。
/// `&self`，既不允许实现者移动对象也不允许修改，对应的闭包特性为`Fn`。
/// `&mut self`，允许实现者修改对象但不允许移动，对应的闭包特性为`FnMut`。
///

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x,
            y,
            radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

pub fn test_method() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    println!("{}", Circle::new(0.0, 0.0, 2.0).area());
}
