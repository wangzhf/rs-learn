
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("function {:?} is called", stringify!($func_name))
        }
    }
}

pub fn test_macro() {
    create_function!(foo);
    foo();
}
