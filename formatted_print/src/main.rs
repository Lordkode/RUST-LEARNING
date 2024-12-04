fn main() {
    //Automatically replaced with any arguments
    println!("{} days", 31);

    //Positionnal arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //As can named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over"
    );

    //Different formatting can be invoked by specifying the format character
    //after a `:`
    println!("Base 10:                 {}", 69420);
    println!("Base 2(binary)         {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    //You can right-justify text with a specified width. This will 
    //output "   1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    //You can pad numbers with extra "*".
    println!("{number:*>5}", number="*");

    //and left-adjust by flipping the sign. This will output "10000"
    println!("{number:0<5}", number=1);

    //You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=10);

    //Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    //FIXME ^ Add the missing argument : "James"

    #[allow(dead_code)]
    //use std::fmt;

    struct Structure(i32);

    // impl fmt::Display for Structure{
    //     fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
    //         write!(f, "{}", self.0);
    //     }
    // }

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // let s = Structure(42);
    // println!("Structure: {}", s);

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
