use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct A(i32);

// 运算符重载
impl Add for A {
    type Output = A;

    fn add(self, rhs: A) -> A {
        A(self.0 + rhs.0)
    }
}

// 运算符重载
impl Sub for A {
    type Output = A;

    fn sub(self, rhs: A) -> A {
        A(self.0 - rhs.0)
    }
}

pub fn test_operator_override() {
    let a1 = A(10i32);
    let a2 = A(5i32);
    let a3 = a1 + a2;
    println!("{}", a3.0);
    let a4 = a1 - a2;
    println!("{}", a4.0);
}

// 测试format!
pub fn test_format_macro() {
    let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子",
            81, "wwww", width = 4, height = 170);
    println!("{}", s);

    println!("{}", format!("{:b}", 2));
    println!("{}", format!("{:?}", "hello"));
}

