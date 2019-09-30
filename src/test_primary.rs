
pub fn test_primary() {

    let a = [1, 2, 3, 4, 5];
    let b = &a[1..3];
    let c = b.first();
    println!("first: {:?}", c.expect("error"));

    // 指针：最底层的是裸指针*const T和*mut T，但解引用它们是不安全的，必须放到unsafe块里。
    let x = 5;
    let raw = &x as *const i32;
    let point_at = unsafe { *raw };
    println!("raw: {:?}", raw);
    println!("point_at: {}", point_at);

}

pub fn test_array() {
    // 创建长度为3类型为i32的数组，并将初始值默认置为0
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    println!("0: {}", array[0]);
    println!("1: {}", array[1]);
    println!("2: {}", array[2]);

    assert_eq!([1, 2], &array[1..]);

    for x in &array {
        println!("{} ", x);
    }
}

pub fn test_vec() {
    // 创建空Vec
    let v: Vec<i32> = Vec::new();
    // 使用宏创建空Vec
    let v: Vec<i32> = vec![];
    // 创建包含5个元素的Vec
    let v = vec![1, 2, 3, 4, 5];
    // 创建十个零
    let v = vec![0; 10];

    let mut v = vec![1, 2];
    v.push(3);
    println!("{}", v.get(0).unwrap());

    let mut v = vec![1, 2];
    println!("{}", v.pop().unwrap());

    // 修改值
    let mut v = vec![1, 3, 5];
    v[1] = v[1] + 1;
    println!("{}", v[1]);

}




