fn main() {
    // the `{}` replaces any arguments, and strings them
    println!("{} days", 31);

    // Without a suffix 31 becomes i32 this can be changed

    //there are various patterns that work with this 
    //positional ones for example
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

    // special formatting can be specified after a ':'
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right align text
    println!("{number:>width$}", number=1, width=6);

    // padding with extra zeros  this will output 000001
    println!("{number:>0width$}", number=1, width=6);

    // it will even make sure correct number of arguments are used!
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // create a structure which contains an 'i32' name it 'Structure'
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated 
    // handling this will not work
    // println("this struct `{}` won't print...", Structure(3));
    


}