use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct CoordenadasTablero(i32, i32);

impl fmt::Display for CoordenadasTablero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}


fn main() {
    println!("Display of struct: {}", Structure(3));
    println!("Display of custom struct: {}", CoordenadasTablero(1,2));
}
