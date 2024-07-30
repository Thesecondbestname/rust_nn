use rand::Rng;
use std::{fmt::Display, ops::Mul};

#[derive(Clone)]
pub struct Matrix<const RSIZE: usize, const CSIZE: usize> {
    field: Box<[[f64; CSIZE]; RSIZE]>,
}
impl<const RSIZE: usize, const CSIZE: usize, const OCSIZE: usize> Mul<&Matrix<CSIZE, OCSIZE>> for Matrix<RSIZE, CSIZE> {
    type Output = Matrix<RSIZE, OCSIZE>;

    fn mul(self, rhs: &Matrix<CSIZE, OCSIZE>) -> Self::Output {
        let mut res: Matrix<RSIZE, OCSIZE> = Matrix::zeros();
        for i in 0..RSIZE{
            for j in 0..OCSIZE{
                let mut sum= 0.;
                for k in 0..CSIZE{
                    sum += self.field[i][k] * rhs.field[k][j];
                }
                res.field[i][j] = sum;
            }
        }
        res
    }
}
impl<const SIZE: usize> Matrix<SIZE, SIZE> {
    pub fn mul(mut self, rhs: Self) -> Self {
        for i in 0..SIZE{
            for j in 0..SIZE{
                self.field[i][j] += rhs.field[j][i];
            }
        }
        self
    }
}
impl Matrix<0, 0> {
    pub fn random<const row_size: usize, const column_size: usize> () -> Matrix<row_size,column_size>{
        let mut rows = [[0f64;column_size];row_size];
        let mut rng = rand::thread_rng();
        for i in 0..row_size{
            for j in 0..column_size{
                rows[i][j] = rng.gen::<f64>() * 2. -1.;
            }
        }
        Matrix::<row_size, column_size> { field: Box::new(rows)}
    }
    pub fn zeros<const row_size: usize, const column_size: usize> () -> Matrix<row_size,column_size>{
        Matrix::<row_size, column_size> { field: Box::new([[0f64;column_size];row_size])}
    }
    
}
impl<const RSIZE: usize, const CSIZE: usize> Matrix<RSIZE, CSIZE> {
    // pub fn random<const rsize: usize, const csize: usize> () -> Matrix<rsize,csize>{
    //     let mut rows = [[0i32;csize];rsize];
    //     for i in 0..rsize{
    //         for j in 0..csize{
    //             rows[i][j] = rand::random();
    //         }
    //     }
    //     Matrix::<rsize, csize> { field: Box::new(rows)}
    // }
    pub fn add(mut self, rhs: &Self) -> Self {
            for i in 0..RSIZE {
                for j in 0..CSIZE {
                    self.field[i][j] += rhs.field[i][j];
                }
            }
            self
    }
    pub fn sub(mut self, rhs: &Self) -> Self {
            for i in 0..RSIZE {
                for j in 0..CSIZE {
                    self.field[i][j] -= rhs.field[i][j];
                }
            }
            self
    }
    pub fn map(mut self, fun: fn(f64) -> f64) -> Self {
        for i in 0..RSIZE {
            for j in 0..CSIZE {
                self.field[i][j] = fun(self.field[i][j])
            }
        }
        self
    }
    pub fn dot_mul(mut self, rhs: &Self) -> Self {
            for i in 0..RSIZE {
                for j in 0..CSIZE {
                    self.field[i][j] *= rhs.field[i][j];
                }
            }
            self
    }
    pub fn transpose(mut self, rhs: &Self) -> Self {
            for i in 0..RSIZE {
                for j in 0..CSIZE {
                    self.field[i][j] = rhs.field[i][j];
                }
            }
            self
    }
}
impl<const rsize: usize, const csize: usize> Display for Matrix<rsize, csize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:->size$}", "", size = 3*csize)?;
        for i in 0..rsize {
            write!(f, "[")?;
            write!(f, "{}", self.field[i][0])?;
            write!(f, "{}", self.field[i].iter().skip(1).map(|a| ", ".to_owned() + &a.to_string() ).collect::<String>())?;
            writeln!(f, "]")?;
        }
        writeln!(f, "{:->size$}", "", size = 3*csize)?;
        Ok(())
    }
}
