use std::time;

pub fn test_vec_init() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(2);
    v1.push(5);
    println!("{:?}", v1);

    let v2 = vec![2, 5];
    println!("{:?}", v2);

    let v3 = vec![1; 3];
    println!("{:?}", v3);

    // 从迭代器生成
    let v4: Vec<i32> = (1..5).collect();
    println!("{:?}", v4);
}

pub fn test_vec_access() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", v[1]);
    // 越界，会panic
    // println!("{}", v[6]);

    // 使用get返回Option，可避免越界
    assert_eq!(v.get(1), Some(&2));
    assert_eq!(v.get(6), None);
}

pub fn test_vec_iterator() {
    let mut vec = vec![1, 3, 5, 7];
    // 获取引用
    for v in &vec {
        print!("{}", v);
    }
    println!();
    // 获取可变引用
    for v in &mut vec {
        print!("{}", v);
    }
    println!();
    // 获取所有权
    for v in vec {
        print!("{}", v);
    }
    println!();
    // vec已经失去所有权，不可再次访问
    // println!("{:?}", vec);
}


// 测试push效率
pub fn test_push_efficiency() {
    let mut v: Vec<usize> = vec![];
    push_1m(&mut v, 50_000_000);
    let mut v: Vec<usize> = vec![];
    v.reserve(50_000_000);
    push_1m(&mut v, 50_000_000);
}

fn push_1m(v: &mut Vec<usize>, total: usize) {
    let e = time::SystemTime::now();
    for i in 1..total {
        v.push(i);
    }
    let ed = time::SystemTime::now();
    println!("time spend {:?}", ed.duration_since(e).unwrap());
}
