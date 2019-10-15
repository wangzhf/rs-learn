
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
    // 解构时，tuple被move了
    // let (x, s) = tuple;
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


// 范围
pub fn test_pattern4() {
    let x = 1;
    match x {
        1...10 => {
            println!("one to ten");
        },
        _ => println!("other"),
    }

    let c = 'w';
    match c {
        'a' ... 'z' => println!("little letter"),
        'A' ... 'Z' => println!("Upper letter"),
        _ => println!("other"),
    }
}

// 多重匹配
pub fn test_pattern5() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("other"),
    }
}

// 使用ref或ref mut从中拿到一个引用，而不是将其remove掉
pub fn test_ref() {
    let mut tuple: (u32, String) = (3, "hello".to_string());
    // 此处使用ref或ref mut取引用，tuple也就不会被move了
    let (x, ref mut t) = tuple;
    println!("{:?}", tuple);

    let s: String = "hello".to_string();
    match s {
        // 解构时使用ref，这样不影响所有权
        ref s => println!("{}", s),
    }
    println!("{}", s);
}


#[derive(Debug)]
struct Person {
    name: Option<String>,
}

// 变量绑定
pub fn test_param_bind() {
    let x = 1u32;
    match x {
        e@ 1...5 | e @ 6...10 => println!("the value is {}", e),
        _ => (),
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: a @ Some(_), .. }) => println!("{:?}", a),
        _ => (),
    }
}

// 后置条件
pub fn test_if() {
    let x = 4;
    let y = false;

    match x {
        // 后置if表达式，等同于伪代码： if y and (x in list(4, 5))
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }
}
