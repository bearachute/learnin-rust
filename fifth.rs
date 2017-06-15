use std::fmt; // Import 'fmt'

// simple
// A structure holding 2 numbers. 'Debug' will be derived so the results 
// can be constrasted with 'Display'
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement 'Display for 'MinMax'.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use 'self.number to refer to each positional data
        // point
        write!(f, "({}, {})", self.0, self.1)
    }
    }

// Define a structure where the fields are nameable
// For Compare
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, imp for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only x and y are denoted
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    println!("Compare structs:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
    small = small_range,
    big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both 'debug and 'display' //were imp but '{:b}' requires // //'fmt::Binary to be imp'd this will // not print
    // println!("What does Point2D look like in binary: {:b}?", point);
}
