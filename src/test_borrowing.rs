

pub fn test_borrowing() {
    let mut x: i32 = 100;
    {
        let y: &mut i32 = &mut x;
        *y += 2;
    }
    println!("{}", x);
}

pub fn test_borrowing2() {
    let mut s: String = String::from("hello");
    {
        let mut y = &mut s;
        // 有可变借用时，可变借用未被释放前不允许访问原变量
        // println!("{}", s);
        y.push('a');
    }
    println!("{}", s);
}


/// 同一作用域下只能有一个可变借用(&mut T)，且被借用的变量本身必须有可变性
pub fn test_borrowing3() {
    // 原变量x可变
    let mut x: Vec<i32> = vec![1, 2, 3];

    // 只能有一个可变借用
    let y = &mut x;
    // 不可再次可变借用
    // let z = &mut x;
    y.push(100);

    println!("{:?}", y);
    y.push(200);
    // 只能在y使用后才能调用，使用前不可调用
    println!("{:?}", x);
}

pub fn test_borrowing4() {
    let mut x: Vec<i32> = vec![1, 3, 5];
    x.push(10);

    {
        let mut y = &mut x;
        y.push(100);

        let z = &mut y;
        z.push(200);
        println!("{:?}", z);
    }

    println!("{:?}", x);
}
