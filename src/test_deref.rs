use std::rc::Rc;

pub fn test_deref() {
    let owned = "hello".to_string();
    foo(&owned);
    let counted = Rc::new(owned);
    foo(&counted);

    // Vec<T> implements Deref<Target=[T]>
    let o2 = vec![1, 3, 4];
    foo2(&o2);
}

fn foo(s: &str) {
    println!("{}", s);
}

fn foo2(s: &[i32]) {
    for &i in s {
        println!("{}", i);
    }
    println!("{:?}", s);
}

struct Person;

impl Person {
    fn new(&self) {
        println!("person");
    }
}

pub fn test_deref2() {
    let p = &&Person;
    p.new();
    (&p).new();
    (&&p).new();
    (&&&&p).new();
}

