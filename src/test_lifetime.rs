
pub fn test_lifetime() {
    let m = "hello";
    let n = "rust";
    foo(m, n);
    foo2(m, n);
}

fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}

// 'b: 'a 显示表明b的生命周期比a的长
fn foo2<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}
