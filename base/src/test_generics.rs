use std::ops::Add;

fn make_pair<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn test_generics1() {
    let couple = make_pair("man", "female");
    println!("{:?}", couple);

    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };
    println!("{:#?}", int_origin);
    println!("{:#?}", float_origin);
}


// 使用关联类型

// use generic parameters
//trait Graph<N, E> {
//    fn has_edge(&self, &N, &E) -> bool;
//    fn edges(&self, &N) -> Vec<E>;
//}
//
//fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
//
//}

// use associated types
trait Graph {
    type N;
    type E;

    fn has_edge(&self, n: &Self::N, e: &Self::E) -> bool;
    fn edges(&self, n: &Self::N) -> Vec<&Self::E>;
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::E) -> usize {
    1usize
}

struct Node;
struct Edge;
struct SimpleGraph;

impl Graph for SimpleGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n: &Node, e: &Edge) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<&Edge> {
        vec![]
    }
}


// 自定义类型
#[derive(Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Add for Point2 {
    type Output = Point2;

    fn add(self, p: Point2) -> Point2 {
        Point2 {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn test_add() {
    println!("{}", add(100i32, 1i32));
    println!("{}", add(100.11f32, 100.22f32));

    let p1 = Point2 { x: 1, y: 1};
    let p2 = Point2 { x: 2, y: 2};
    println!("{:#?}", add(p1, p2));
}

// 基于上面Point2的改造，直接限制Point中的类型
#[derive(Debug)]
struct Point3<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point3<T> {
    type Output = Point3<T>;

    fn add(self, p: Point3<T>) -> Point3<T> {
        Point3 {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add3<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn test_add3() {
    let p1 = Point3 { x: 1.1f32, y: 1.1f32 };
    let p2 = Point3 { x: 2.1f32, y: 2.1f32 };
    println!("{:?}", add3(p1, p2));

    let p3 = Point3 { x: 1i32, y: 1i32 };
    let p4 = Point3 { x: 2i32, y: 2i32 };
    println!("{:?}", add3(p3, p4));
}
