mod square;
mod cg_sparse;

use square::*;
use cg_sparse::*;

//use std::fs::File;
//use std::io::prelude::*;
//
//use std::vec::Vec;

use gnuplot::*;


fn main() {
    let size = 200;
    let mut domain = Square::new((0.0, 0.0), 1.0, size);

    domain.set_upper_bc_dirichlet(|x| (-2.0*x).exp());
    domain.set_left_bc_dirichlet(|x| (2.0*x).cos());
    domain.set_lower_bc_dirichlet(|x| (-2.0*x).exp()*2.0_f64.cos());
    domain.set_right_bc_dirichlet(|x| (2.0*x).cos()*(-2.0_f64).exp());

    println!("BC set!");

    print!("{}", domain.domain.to_string().as_str());

    let (m, bc_rhs) = domain.laplace();

    println!("Matrix assembled!");
    
    //print!("{}", m.to_string().as_str());
    //print!("{}", bc_rhs.to_string().as_str());

    //let s = m.to_string();
    //let data = s.as_bytes();

    //let mut buffer = File::create("matrix.txt").expect("Exists");
    //buffer.write_all(data);

    //let s = bc_rhs.to_string();
    //let data = s.as_bytes();

    //let mut buffer = File::create("rhs.txt").expect("Exists");
    //buffer.write_all(data);

 //   let res = m.cholesky().expect("Decomp. failed").solve(&bc_rhs);
    let res = conjugate_gradient(&m, &bc_rhs, 1e-6, None);

    println!("Matrix solved!");
    
    let mut fg = Figure::new();
    
    fg.axes3d()
    	.set_title("Surface fg3.1", &[])
    	.surface(res.iter(), size - 2, size - 2, None, &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
        .set_view_map();
    
    fg.show().unwrap();
}



/*
 *
    let s = m.to_string();
    let data = s.as_bytes();

    let mut buffer = File::create("matrix.txt").expect("Exists");
    buffer.write_all(data);

    let s = rhs.to_string();
    let data = s.as_bytes();

    let mut buffer = File::create("rhs.txt").expect("Exists");
    buffer.write_all(data);
 *
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

