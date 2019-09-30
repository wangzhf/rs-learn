
// if 是一个表达式
#[allow(dead_code)]
pub fn test_if() {
    let y = ();
    let x = 0;
    let z: i32 = if x == 5 { 10 } else { 15 };
    println!("{:?}", y);
    println!("{}", z);
}

#[allow(dead_code)]
pub fn test_for() {
    for i in 0..10 {
        print!("{}, ", i);
    }
    println!(" ");
    for i in 0..=2 {
        print!("{}, ", i);
    }
    println!(" ");
    let v = vec![1, 2, 3, 5, 7];
    for i in v.iter() {
        print!("{}, ", i);
    }
    println!(" ");
    let a = [1; 10];
    for i in a.iter() {
        print!("{}, ", i);
    }
}

struct Point3 {
    x: i32,
    y: i32,
}

pub fn test_match() {
    let day = 5;
    match day {
        // | 用于匹配多个值
        0 | 6 => println!("weekend"),
        // ... 用于匹配一个范围，包含最后一个值
        1 ... 5 => println!("workday"),
        // match强制进行穷尽检查，必须覆盖所有可能性
        _ => println!("invalid"),
    }

    // 如果需要得到|或者...匹配的值，可以使用@绑定变量
    let x = 5;
    match x {
        e @ 1 ... 5 => println!("got a range element {} ", e),
        _ => println!("anything"),
    }


    // 使用ref获取引用
    let xx = 5;
    let mut yy = 5;
    match xx {
        ref r => println!("Got a reference to {}", r),
    }

    match yy {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    // match解构元组
    let pair = (0, -2);
    match pair {
        (0, y) => println!("x is `0` and y is `{}`", y),
        (x, 0) => println!("`x` is `{}` and y is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }

    // match 解构结构体或者枚举
    let origin = Point3 { x: 0, y: 0 };
    match origin {
        Point3{ x, .. } => println!("x is {}", x),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five."),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }


    let number = Some(7);
    let mut optional = Some(0);
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number!");
    }

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("i is {:?}, Try again!", i);
            optional = Some(i + 1);
        }
    }
}
