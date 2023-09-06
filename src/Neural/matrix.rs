#![allow(non_snake_case)]

use rand::Rng;

#[derive(Debug)]
pub struct Matrix {
    data: Vec<f32>,

    rows: u32,
    cols: u32,
    total: u32,
}

impl Matrix {
    pub fn new(rows: u32, columns: u32) -> Matrix {
        return Matrix {
            data: Vec::new(),
            rows: rows,
            cols: columns,
            total: rows * columns
        }
    }

    fn get(&self, x: u32, y: u32) -> f32 {
        return self.data[usize::try_from(x + y * self.rows).unwrap()]
    }

    fn get_mut(&mut self, x: u32, y: u32) -> &mut f32 {
        return self.data.get_mut(usize::try_from(x + y * self.rows).unwrap()).unwrap();
    }

    pub fn MUL(&mut self, a: &Matrix, b: &Matrix) {
        if (a.cols != b.rows) {
            panic!("columns and rows of matrices do not match");
        }

        let sharedDim = a.cols;

        for r in 0..a.rows {
            for c in 0..b.cols {
                let mut total: f32 = 0.0;
                for d in 0..sharedDim {
                    total += a.get(r, d) * b.get(d, c);
                }

                *self.get_mut(r, c) = total;
            }
        }
    }

    pub fn Hadamard(&mut self, a: &Matrix) {
        if self.rows != a.rows || self.cols != a.cols {
            panic!("matrices must have the same dimensions to Hadamard");
        }

        for r in 0..self.rows {
            for c in 0..self.cols {
                *self.get_mut(r, c) *= self.get(r, c);
            }
        }
    }

    pub fn ADD(&mut self, a: &Matrix) {
        if self.rows != a.rows || self.cols != a.cols {
            panic!("matrices must have the same dimensions to ADD");
        }

        for r in 0..self.rows {
            for c in 0..self.cols {
                *self.get_mut(r, c) += self.get(r, c);
            }
        }
    }

    pub fn ADD_COL_VEC(&mut self, a: &Matrix) {
        if self.rows != a.rows || a.cols != 1 {
            panic!("matrices must have the same number of rows and have one column to ADD_COL_VEC");
        }

        for c in 0..self.cols {
            for r in 0..self.rows {
                *self.get_mut(r, c) += self.get(r, 0);
            }
        }
    }

    pub fn SUB(&mut self, a: &Matrix) {
        if self.rows != a.rows || self.cols != a.cols {
            panic!("matrices must have the same dimensions to SUB");
        }

        for r in 0..self.rows {
            for c in 0..self.cols {
                *self.get_mut(r, c) -= self.get(r, c);
            }
        }
    }

    pub fn Randomize(&mut self)
    {
        let mut rng = rand::thread_rng();

        for i in 0..self.total {
            self.data[usize::try_from(i).unwrap()] = rng.gen();
        }
    }
}