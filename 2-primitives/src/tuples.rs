    use std::fmt;
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let first_row = (self.0, self.1);
            let second_row = (self.2, self.3);
            write!(f, "{:?}\n{:?}", first_row, second_row)
        }
    }


    pub fn print_matrix() {
        let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
        println!("Matrix:\n{}", matrix);
    }

    pub fn transpose_matrix() {
        let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
        let transposed = Matrix(
            matrix.0,
            matrix.2,
            matrix.1,
            matrix.3);
        println!("Matrix:\n{}", transposed);
    }
