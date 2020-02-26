mod square;
use square::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let domain = Square::new((0.0, 0.0), 1.0, 10);

    let (m, rhs) = domain.laplace();

    let s = m.to_string();
    let data = s.as_bytes();

    let mut pos = 0;
    let mut buffer = File::create("matrix.txt").expect("Exists");

    buffer.write_all(data);
}

/*
    println!("{:?}", domain);
    println!("{:?}", domain.x());
    println!("{:?}", domain.grid().collect::<Vec<(usize, usize)>>());
 *

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

