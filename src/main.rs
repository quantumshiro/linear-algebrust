use algebrust::matrix::Matrix;

fn main() {
    let mut m2 = Matrix::from_string("1 2 3; 4 5 6; 7 8 9");
    m2.print();
    m2.apply(|x| x + 2.0);
    m2.print();
}

