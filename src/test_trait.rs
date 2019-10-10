use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

pub fn test_trait1() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    print_area(c);
    let s = Square { x: 0.0, y: 0.0, side: 3.0 };
    print_area(s);
}


/// 测试多个限定
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}
/// 使用where指定类型
fn bar<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}


/// 特性继承
trait Foo {
    fn foo(&self);

    // default method
    fn bar(&self) {
        println!("bar from Foo.");
    }
}

/// 继承
trait FooBar : Foo {
    fn foobar(&self);

    fn bar(&self);
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) {
        println!("foo from Foo");
    }

    fn bar(&self) {
        println!("bar from Baz Foo");
    }
}

impl FooBar for Baz {
    fn foobar(&self) {
        println!("foobar from FooBar");
    }

    fn bar(&self) {
        println!("bar from FooBar");
    }
}

pub fn test_trait2() {
    let baz = Baz {};
    /// 两个不同特性具有相同的名称，需要使用通用函数调用方法
    // baz.bar();
    Foo::bar(&baz);
    FooBar::bar(&baz);
    baz.foo();
    baz.foobar();
}


/// 关于实现特性的几条限制：
/// - 如果一个特性不在当前作用域内，它就不能被实现。
/// - 不管是特性还是impl，都只能在当前的包装箱内起作用。
/// - 带有特性约束的泛型函数使用单态化实现 (monomorphization)， 所以它是静态派分的 (statically dispatched)。


