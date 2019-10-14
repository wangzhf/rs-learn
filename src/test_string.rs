

pub fn test_str() {
    let x = "Hello";
    let y: &'static str = "Hello";

    assert_eq!(x, y);

    // 支持转义
    let z = "foo
bar";
    let w = "foo\nbar";
    assert_eq!(z, w);

    // 使用r来避免转义
    let d: &'static str = r"abc \n abc";
    let c: &'static str = "abc \\n abc";
    assert_eq!(d, c);
}

pub fn test_string() {
    let x: &'static str = "hello";
    // 将&str转变为String
    let mut y: String = x.to_string();
    println!("{}", y);

    y.push_str(", world");
    println!("{}", y);

    // 将String转变为&str
    let s = "Hello".to_string();
    // String实现了Deref，使用*来解引用
    let t = &*s;
    println!("{}", t);

    // 将UTF-8编码的字节数组转换成String
    let miao = vec![229, 150, 181];
    let meow = String::from_utf8(miao).unwrap();
    assert_eq!("喵", meow);

    // 访问
    let x = "今天气晴朗".to_string();
    for i in x.as_bytes() {
        print!("{} ", i);
    }
    println!("");
    for i in x.chars() {
        print!("{} ", i);
    }
    println!("");
    // nth取第几个字符
    println!("{}", x.chars().nth(2).unwrap());
    println!("{}", x);
}
