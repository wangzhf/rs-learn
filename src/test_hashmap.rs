use std::collections::HashMap;

pub fn test_hashmap() {

    // 初始化
    let mut come_from = HashMap::new();
    come_from.insert("first", "hello");
    come_from.insert("second", "rust");
    come_from.insert("third", "!");

    // 查找
    if !come_from.contains_key("fourth") {
        println!("We found {} words, but not found the fourth", come_from.len());
    }
    if come_from.contains_key("first") {
        println!("We found the word: {}", come_from.get("first").unwrap());
    }


    // 根据key删除元素
    come_from.remove("third");
    println!("Left {} words", come_from.len());

    // 利用get的返回值判断元素是否存在
    let orders = ["fifth", "second"];
    for order in &orders {
        match come_from.get(order) {
            Some(value) => println!("{} is {}", order, value),
            None => println!("not exist: {}", order),
        }
    }

    // 遍历
    for (order, value) in &come_from {
        println!("{}: {}", order, value);
    }

}

// entry

pub fn test_entry() {
    let mut letters = HashMap::new();

    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }

    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters.get(&'y'), None);
}
