use std::fmt;

struct Complex(i32, i32);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} + {}i", self.0, self.1)
    }
}

fn main() {
    println!("Número imaginario: {}", Complex(1,3));
}


