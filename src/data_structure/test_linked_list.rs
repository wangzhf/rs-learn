use List::*;


enum List {
    // Cons：包含一个元素和一个指向下一个节点的指针的元组结构
    Cons(u32, Box<List>),
    // Nil：表示一个链表节点的末端
    Nil,
}


impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(3);
        list = list.prepend(5);
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}
