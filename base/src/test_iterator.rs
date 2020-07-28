use std::collections::HashMap;

pub fn test_for() {
    // (1..10)是一个迭代器
    for i in 1..10 {
        println!("{}", i);
    }

    // 可以对迭代器调用next()方法
    println!("{}", (1..10).next().unwrap());
}


pub fn test_vec() {
    let v = vec![1, 3, 5];
    // vec并没有直接实现Iterator，而是实现了IntoIterator
    for i in v {
        println!("{}", i);
    }
    // vec并没有next()方法，所以此方法不可行
    // println!("{}", v.next());
}


// 测试消费者
// 迭代器负责生产，而消费者则负责将生产出来的东西最终做一个转化，如collect()

pub fn test_collect() {
    // collect方法只会将迭代器收集到一个实现了FromIterator的类型中，
    // 但是实现了这个trait的类型有很多，比如（Vec, HashMap等），
    // 因此collect没有一个上下文来判断应该将v按照什么方式来收集。
    // let v = (1..20).collect();

    // 方式一：显式表明v的类型：
    let v: Vec<_> = (1..20).collect();

    // 方式二：显式指定collect调用时的类型
    let v = (1..20).collect::<Vec<_>>();

    // 测试fold: 类似于MapReduce中的Reduce
    // 1u64指定类型为u64，默认为i32的话，会溢出
    let m = (1..20).fold(1u64, |mul, x| {
        println!("{}: {}", mul, x);
        mul * x
    });
    println!("The final value is : {}", m);

    // 等价于：
    let mut result = 1u64;
    for i in 1..20 {
        result *= i;
    }
    println!("the value is : {}", result);
    assert_eq!(m, result);
}


// 测试适配器
// 适配器是将生产者生产的东西再组装，返回一个新的迭代器，可以一直用链式请求一直写下去

pub fn test_adapter() {
    // map
    let v: Vec<i32> = (1..20).map(|x| x + 1).collect();
    println!("{:?}", v);

    // filter
    let v: Vec<i32> = (1..20).filter(|x| x % 2 == 0).collect();
    println!("{:?}", v);
}


// 测试其他适配器和消费者

pub fn test_other() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // take: 取前几个元素
    let v_take = v.iter().cloned()
        .take(2)
        .collect::<Vec<_>>();
    assert_eq!(v_take, vec![1, 2]);

    // skip: 跳过前几个元素
    let v_skip = v.iter().cloned()
        .skip(5)
        .collect::<Vec<_>>();
    assert_eq!(v_skip, vec![6, 7, 8]);


    // zip: 适配器，将两个迭代器内容压缩到一起，形成Iterator<Item=(value_from_A, value_from_B)>
    // 如果数量不等，以数量少的为主
    let names = vec!["zhangsan", "lisi", "wangwu", "zhaoliu"];
    let scores = vec![100, 70, 90];
    let score_map: HashMap<_, _> = names.iter()
        .zip(scores)
        .collect();
    println!("{:?}", score_map);


    // enumerate: 显示迭代器下标
    let v = vec![1u64, 2, 3, 4, 5];
    let val = v.iter()
        .enumerate()
        .collect::<Vec<_>>();
    println!("{:?}", val);


    // find
    let v = vec!["hello", "rust", "golang", "vue", "rust"];
    let v_find = v.iter()
        .find(|&&val| val.eq("rust"));
    println!("{:?}", v_find); // Some("rust")

    // position
    let v_position = v.iter()
        .position(|&val| val.eq("rust"));
    println!("{:?}", v_position); // Some(1)

    // all
    let v_all = v.iter()
        .all(|&val| val.eq("rust"));
    println!("{:?}", v_find); // Some("rust")

    // any
    let v_any = v.iter()
        .any(|&val| val.eq("rust"));
    println!("{:?}", v_any); // true


    let v = vec![2, 4, 6, 8];
    // max
    let max = v.iter()
        .max();
    let min = v.iter()
        .min();
    println!("max: {}, min: {}", max.unwrap(), min.unwrap());


    // 水仙花数
    let mut nn: Vec<i32> = generate_narcissistic_number(10);
    println!("{:?}", nn);
    nn.reverse();
    println!("after reverse: {:?}", nn);
}


// 生成水仙花数
fn generate_narcissistic_number(len: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 100..1000 {
        let first = i / 100;
        let second = i % 100 / 10;
        let third = i % 100 % 10;
        if first * first * first + second * second * second + third * third * third == i {
            v.push(i);
        }
    }
    v
}

