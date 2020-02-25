struct Linspace {
    from: f64,
    to: f64,
    n: u32
}

impl Linspace {
    fn new(from: f64, to: f64, n: u32) -> Linspace {
        Linspace { from: from, to: to, n: n }
    }

    fn dx(&self) -> f64 {
        (self.to - self.from)/(self.n - 1) as f64
    }

    fn gen(&self) -> Vec<f64> {
        (0..(self.n))
            .map(|i| self.from + self.dx()*i as f64)
            .collect()
    }
}


// use nalgebra::sparse::CsMatrix;
use nalgebra::{DMatrix, DVector};

fn main() {
    let a = Linspace::new(-1.0, 21.0, 30);
    println!("dx: {}", a.dx());
    println!("{:?}", a.gen());
    println!("{}", a.gen().len());

    let (nx, ny) = (3, 3);
    let mut mx = DMatrix::identity(nx, nx);
    let mut my = DMatrix::identity(ny, ny);

    for i in 0..nx {
        mx[(i,i)] = 2;
    }
    for i in 0..(nx-1) {
        mx[(i+1, i)] = -1;
        mx[(i, i+1)] = -1;
    }
    for i in 0..ny {
        my[(i,i)] = 2;
    }
    for i in 0..(ny-1) {
        my[(i+1, i)] = -1;
        my[(i, i+1)] = -1;
    }
    let m = mx.kronecker(&my);
    let (I, J) = m.shape();
    for i in 0..I {
        for j in 0..J {
            print!("{:?} ", m[(i,j)]);
        }
        println!("")
    }
}
