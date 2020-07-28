
// 使用此for时无法获取索引
pub fn test_for_int() {
    for x in 0..=10 {
        println!("{}", x);
    }
}

// 获取索引方式
pub fn test_for_index() {
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
}

pub fn test_line() {
    let lines = "Content of line one
    Content of line two
    Content of line three
    Content of line four".lines();

    for (lineNumber, line) in lines.enumerate() {
        println!("{}: {}", lineNumber, line);
    }
}

pub fn test_while() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }
}

// 测试未初始化变量能否在while中使用
pub fn test_while_init() {
    let mut x;
    let mut done = false;

    while !done {
        x = 1;
        println!("{}", x);
        if x == 1 {
            done = true;
        }
    }
}

// 测试未初始化变量能否在loop中使用
pub fn test_loop_init() {
    let mut x;
    loop {
        x = 1;
        if x == 1 {
            break;
        }
    }
    println!("end: {}", x);
}

// 测试break
pub fn test_break() {
    let mut x = 5;
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; }
    }
}

// 测试continue
pub fn test_continue() {
    for x in 0..10 {
        if x % 2 == 0 {
            continue;
        }
        println!("{}", x);
    }
}

// 测试label，配合continue和break作用于指定的循环
pub fn test_label() {
    // continue
    'outer_label: for x in 0..10 {
        'inner_label: for y in 0..10 {
            if x % 2 == 0 { continue 'outer_label; }
            if y % 2 == 0 { continue 'inner_label; }
            println!("x: {}, y: {}", x, y);
        }
    }

    // break
    'outer_label: for x in 0..5 {
        'inner_label: for y in 0..5 {
            if y != 0 && y % 3 == 0 {
                break 'inner_label;
            }
            println!("x: {}, y: {}", x, y);
        }
    }
}
