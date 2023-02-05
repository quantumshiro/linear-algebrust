pub mod matrix {
    use std::fs;

    #[derive(Debug)]
    pub struct Matrix {
        pub rows: usize,
        pub cols: usize,
        pub data: Vec<Vec<f64>>,
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            let data: Vec<Vec<f64>> = vec![vec![0.0]; rows];
            Matrix { rows, cols, data }
        }

        pub fn from_file(path: &str) -> Matrix {
            let content: String = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
            let mut matrix: Vec<Vec<f64>> = Vec::new();
            for rows in content.lines() {
                let mut row: Vec<f64> = Vec::new();
                let entries: Vec<&str> = rows.split_whitespace().collect();
                for ent in entries {
                    row.push(ent.parse::<f64>().unwrap());
                }
                matrix.push(row);
            }
            let r: usize = matrix.len();
            let c: usize = matrix[0].len();
            Matrix {
                rows: r,
                cols: c,
                data: matrix,
            }
        }

        pub fn from_string(input: &str) -> Matrix {
            let mut data: Vec<Vec<f64>> = Vec::new();
            let rows: Vec<&str> = input.split(';').collect();
            for row in rows {
                let entries: Vec<&str> = row.split_whitespace().collect();
                let mut tmp_row: Vec<f64> = Vec::new();
                for ent in entries {
                    tmp_row.push(ent.parse::<f64>().unwrap());
                }
                data.push(tmp_row);
            }
            let n_r: usize = data.len();
            let n_c: usize = data[0].len();
            Matrix {
                rows: n_r,
                cols: n_c,
                data,
            }
        }

        pub fn copy(&self) -> Matrix {
            let mut data: Vec<Vec<f64>> = Vec::new();
            for row in &self.data {
                data.push(row.to_vec());
            }
            Matrix {
                rows: self.rows,
                cols: self.cols,
                data,
            }
        }

        pub fn print(&self) {
            self.data.iter().for_each(|v| println!("{:?}", v));
            println!();
        }

        pub fn identity(&mut self) {
            if self.rows != self.cols {
                panic!("Not a square matrix");
            }
            for i in 0..self.rows {
                self.data[i][i] = 1.0;
            }
        }

        pub fn apply(&mut self, f: impl Fn(f64) -> f64) {
            self.data = self.data.iter()
                                    .map(|row| row.iter().map(|x| f(*x)).collect())
                                    .collect();
        }
    }
}
