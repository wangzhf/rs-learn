use std::io;
use std::io::Write;

pub fn read_input() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("You typed: {}", input);
    Ok(())
}


pub fn read_input2() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("WTF!");
    println!("You also typed: {}", input.trim());
}


pub fn out() {
    print!("this ");
    print!("will ");
    print!("be ");
    print!("wonderful!\n");
}

pub fn read_and_out() {
    print!("Please input a string: ");
    // 如果不flush的话，会导致提示信息跟输入内容同时输出
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read!");
    print!("The String you input is: {}", input);
}
