// write!(f "{}", value)?
// no idea
use std::fmt; // import 'fmt' module

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract value using tuple indexing
        // and create ref to vec
        let vec = &self.0;

        write!(f, "[")?;

        // Iteration over vec in v while
        // enumerating the count in count
        for (count, v) in vec.iter().enumerate() {
            // For every element cept first add a comma
            // Use the ? operator or try! to return on error
            if count != 0 { write!(f, ": ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }

}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v);
}
