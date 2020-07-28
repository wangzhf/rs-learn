

pub fn test_closure() {

    let plus_two = | x | {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };

    assert_eq!(4, plus_two(2));
}


pub fn test_closure2() {
    // 实现Copy特性
    let mut num = 5;
    {
        // 不使用move时，实际上在闭包中获得一个num的可变引用
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);
}

pub fn test_closure3() {
    // 实现Copy特性
    let mut num = 5;
    {
        // 使用move时，实际上是把num move进了闭包，因为i32类型实现了Copy特性，
        // 因此move后，实际上是一个独立的变量
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(5, num);
}

pub fn test_closure4() {
    // 未实现Copy特性
    let mut s = String::from("hello");
    {
        // 未使用move，闭包中获得的是s的可变引用
        let mut add_char = |n: char| s.push(n);
        add_char('c');
    }
    println!("{}", s);
}

pub fn test_closure5() {
    // 未实现Copy特性
    let mut s = String::from("hello");
    {
        // 使用move，将s所有权移进闭包，外部s失去所有权
        let mut add_char = move |n: char| s.push(n);
        add_char('c');
    }
    // s所有权已经被move进闭包，此处不可用
    // println!("{}", s);
}


// 测试将闭包作为参数和返回值

fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32{
    some_closure(1)
}

pub fn test_closure_as_parameter() {
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);
}


// 函数指针和闭包
fn call_with_me2(some_closure: & dyn Fn(i32) -> i32) ->i32 {
    some_closure(1)
}

fn add_one(i: i32) -> i32 {
    i + 1
}

pub fn test_fn_point_closure() {
    let answer = call_with_me2(&add_one);
    assert_eq!(2, answer);
}


// 闭包作为返回值
// 在rust中，需要知道返回值的大小，但闭包无法确定大小，所以需要使用引用，引用的大小是固定的
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

pub fn test_closure_as_return_value() {
    // let f = factory();
    // let answer = f(1);
    let answer = factory()(1);
    assert_eq!(6, answer);
}

