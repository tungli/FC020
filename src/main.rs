use nalgebra::{DMatrix, DVector};
use itertools::Itertools;

struct Linspace {
    from: f64,
    to: f64,
    n: usize
}

impl Linspace {
    fn new(from: f64, to: f64, n: usize) -> Linspace {
        Linspace { from: from, to: to, n: n }
    }

    fn dx(&self) -> f64 {
        (self.to - self.from)/(self.n - 1) as f64
    }

    fn gen(&self) -> impl Iterator<Item = f64> + '_ {
        (0..(self.n))
            .map(move |i| self.from + self.dx()*i as f64)
    }
}

#[derive(Debug)]
struct Rectangle {
    l_corner: (f64, f64),
    u_corner: (f64, f64),
    n: usize
}

impl Rectangle {
    fn new(a: (f64, f64), b: (f64, f64), n: usize) -> Rectangle {
        if a.0 > b.0 || a.1 > b.1 {
            panic!("First argument must be the left-most, lowest point");
        }
        Rectangle {l_corner: a, u_corner: b, n: n}
    }

    fn x(&self) -> Vec<f64> {
            Linspace::new(self.l_corner.0, self.u_corner.0, self.n)
                .gen()
                .collect()
    }

    fn y(&self) -> Vec<f64> {
            Linspace::new(self.l_corner.1, self.u_corner.1, self.n)
                .gen()
                .collect()
    }

    fn grid(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.n).cartesian_product(0..self.n)
    }
}

// use nalgebra::sparse::CsMatrix;
use num_traits::Float;

fn main() {
    let domain = Rectangle::new((0.0, 0.0), (1.0, 1.0), 5);
    println!("{:?}", domain);
    println!("{:?}", domain.x());
    println!("{:?}", domain.grid().collect::<Vec<(usize, usize)>>());
}

/*
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
    for i in m.row_iter() {
        for j in i.column_iter() {
            print!("{:?} ", j[0]);
        }
        println!("");
    }
    println!("{}", 3.21.cos());
*/

