mod matrix;
mod network;

const LAYERS: [usize; 4] = [4,5,6,2];

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    #[test]
    fn initialize() {
        let matrix: Matrix<5, 4> = Matrix::zeros();
        println!("{}", matrix);
    }
    #[test]
    fn random() {
        let matrix = Matrix::random::<4,4>();
        println!("{}", matrix);
    }
    #[test]
    fn add() {
        let matrix: Matrix<7, 4> = Matrix::zeros();
        let matrix2: Matrix<7, 4> = Matrix::zeros();
        println!("{}", matrix.add(&matrix2));
    }
    #[test]
    fn mul() {
        let matrix: Matrix<5, 4> = Matrix::random();
        let matrix2: Matrix<4, 4> = Matrix::random();
        println!("{}", matrix * &matrix2);
    }
    #[test]
    fn transpose() {
        let matrix: Matrix<7, 3> = Matrix::random();
        let matrix2: Matrix<7, 3> = Matrix::random();
        println!("{}", matrix.transpose(&matrix2));
    }
    #[test]
    fn dot_mul() {
        let matrix: Matrix<3, 4> = Matrix::random();
        let matrix2: Matrix<3, 4> = Matrix::random();
        println!("{}", matrix.dot_mul(&matrix2));
    }
}
