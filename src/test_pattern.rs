
pub fn test_pattern() {
    let x = 1;
    let c = 'c';

    match c {
        // 此时x会被覆盖成c
        x => println!("x: {}, c: {}", x, c),
    }

    // 此时x会被还原为1
    println!("x: {}", x);
}

// 测试解构struct

struct Point {
    x: i64,
    y: i64,
}

pub fn test_pattern2() {
    let point = Point { x: 0, y: 0 };
    match point {
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }

    // 对字段重命名
    match point {
        Point { x: x1, y: y1} => println!("x1: {}, y1: {}", x1, y1),
    }

    // 省略字段
    match point {
        Point { x, .. } => println!("x: {}", x),
    }
}

pub fn test_pattern3() {
    let tuple: (u32, String) = (5, String::from("five"));
    let (x, s) = tuple;
    // String 未实现Copy，所以tuple被整体move了
    println!("Tuple is : {:?}", tuple);

    let s: String = "Hello".to_string();
    println!("{}", s);
    println!("{:?}", s);

    let t = (5, String::from("five"));
    // 忽略String类型，而u32实现了Copy，所以tuple不会被move
    let (x, _) = t;
    println!("Tuple is {:?}", t);
}
