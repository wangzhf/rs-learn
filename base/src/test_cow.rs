use std::borrow::Cow;

pub fn test_cow() {
    // to_mut
    let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    let hello = cow.to_mut();
    assert_eq!(hello, &[1, 2, 3]);

    // to_owned
    let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    let hello = cow.into_owned();
    assert_eq!(vec![1, 2, 3], hello);
    // moved
    // println!("{:?}", cow);
}

fn remove_spaces(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());

    for c in input.chars() {
        if c != ' ' {
            buf.push(c);
        }
    }

    buf
}

fn remove_spaces2(input: &str) -> Cow<str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    Cow::Borrowed(input)
}

pub fn test_cow3() {
    let s = remove_spaces("hell o worl d");
    println!("{}", s);

    let s = remove_spaces2(" he llo wo old");
    println!("{:?}", s);
}
