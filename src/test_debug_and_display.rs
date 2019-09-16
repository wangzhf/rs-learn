use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {

    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

// Display 自定义输出样式
pub fn test_display() {
    let s = Structure(3);
    println!("{}", s);
}


#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// {:b} 转二进制输出时需要实现Binary trait
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "x: {}, y: {}", self.x.to_bits(), self.y.to_bits())
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub fn test_debug_and_display() {
    let minmax = MinMax(0, 14);
    println!("Compare structures: ");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points: ");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // {:b} 无法直接转二进制，需要Point2D实现Binary trait
    println!("What does Point2D look like in binary: {:b} ?", point);

    let c = Complex { real: 3.3, imag: 7.2 };
    println!("Compare complex: ");
    println!("Display: {}", c);
    println!("Debug: {:?}", c);
}
