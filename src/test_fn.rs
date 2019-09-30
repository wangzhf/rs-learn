
pub fn test_fn() {

    let x: i32 = diverges();
    let y: String = diverges();
    println!("x is {}", x);
    print!("y is {}", y);

}

// 返回类型`!`可以用作任何类型
fn diverges() -> ! {
    panic!("This function never returns!");
}

pub fn test_closure() {
    let num = 5;
    let plus_num = |x: i32| x + num;
    let v = plus_num(5);
    println!("v is {}", v);

    let mut step = 10;
    let mut multiply_num = move |x: i32| -> i32 {
        step *= x;
        step
    };
    let m = multiply_num(2);
    println!("m is {:?}", m);
}