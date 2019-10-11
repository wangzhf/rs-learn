
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
