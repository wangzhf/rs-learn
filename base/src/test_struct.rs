use std::cell::Cell;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// tuple struct
struct Color(u8, u8, u8);

// 元组结构体的构造函数能当做函数
#[derive(Debug)]
struct Digit(i32);

// newtype: 只有一个元素的元组结构体
struct Inches(i32);

// unit-like struct: 类单元结构体
#[derive(Debug)]
struct EmptyStruct;

pub fn test_struct() {
    let point = Point { x: 9, y: 1 };
    println!("{:?}", point);

    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green;
    println!("red: {}, green: {}, blue: {}", red, green, blue);


    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    println!("{:?}", d[1].0);

    let length = Inches(10);
    let Inches(inter_length) = length;
    println!("{}", inter_length);

    let empty = EmptyStruct;
    println!("{:?}", empty);
}

// ..可以从其他结构体拷贝一些值或者在解构时忽略一些域
#[derive(Default)]
#[derive(Debug)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

pub fn test_struct2() {
    let origin = Point3d::default();
    println!("{:?}", origin);

    let point = Point3d { y: 1, ..origin };
    println!("{:?}", point);

    let Point3d { x: x0, y: y0, .. } = point;
    println!("x: {}, y: {}", x0, y0);
}


// 结构体域可变性, 不能使用mut，可以使用Cell模拟
#[derive(Debug)]
struct Point4 {
    x: i32,
    y: Cell<i32>,
}

pub fn test_struct3() {
    let point = Point4 { x: 3, y: Cell::new(5) };
    println!("{:?}", point);
    point.y.set(7);
    println!("{:?}", point);
}
