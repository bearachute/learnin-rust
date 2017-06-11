// Derive the `fmt::Debug` implementation for `Structure`
// contains a single `i32`
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure inside of a structure so deep 
// inception anyone?
#[derive(Debug)]
struct Deep(Structure);

// Printing with `{:?} is similar to with `{}`
fn main() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
    "Slater",
    "Christian",
    actor="actor's");

// `Structure` is now printable dawg!
println!("Now {:?} will print", Structure(3));

// But the problem is no control, what if I just wanted the 7
// Many wow, many what if
println!("Now {:?} will print!", Deep(Structure(7)));
}