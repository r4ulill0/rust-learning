use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let vector = &self.0;
        write!(f, "\n[")?;

        for (count, value) in vector.iter().enumerate() {
            if count != 0 {write!(f, ",")?;}
            write!(f, "\n {0}: {1}", count, value)?;
        }
        write!(f, "\n]")

    }
}

fn main() {
    let vector = List(vec![1,2,3,4,5,6]);
    println!("Veamos un vector en accion {}", vector);
}
