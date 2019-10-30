
pub fn test_from() {
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);
    if string == other_string {
        println!("it is same.");
    }
}

fn is_hello<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

pub fn test_into() {
    let s = "hello".to_string();
    // String 类型实现了Into<Vec<u8>>
    is_hello(s);
}

struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Person {
        Person {
            name,
        }
    }
}


