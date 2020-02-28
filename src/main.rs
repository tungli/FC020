mod square;
mod cg_sparse;

use square::*;
use cg_sparse::*;

//use std::fs::File;
//use std::io::prelude::*;

use gnuplot::*;


fn main() {
    let size = 200;
    let mut domain = Square::new((0.0, 0.0), 1.0, size);

    domain.set_upper_bc_dirichlet(|x| (-2.0*x).exp());
    domain.set_left_bc_dirichlet(|x| (2.0*x).cos());
    domain.set_lower_bc_dirichlet(|x| (-2.0*x).exp()*2.0_f64.cos());
    domain.set_right_bc_dirichlet(|x| (2.0*x).cos()*(-2.0_f64).exp());

    println!("BC set!");

    //print!("{}", domain.domain.to_string().as_str());

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
    let res = conjugate_gradient(&m, &bc_rhs, 1e-8, None);

    println!("Matrix solved!");

    for i in 1..(size-1) {
        for j in 1..(size-1) {
            let k = domain.sub2ind_interior((i,j));
            domain.domain[(i,j)] = res[k];
        }
    }

    let mut fg = Figure::new();
    
    fg.axes3d()
    	.set_title(format!("Size: {}", size).as_str(), &[])
    	.surface(domain.domain.transpose().iter(), size, size, Some((0.0, 0.0, 1.0, 1.0)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
        .set_x_range(Fix(0.0), Fix(1.0))
        .set_y_range(Fix(0.0), Fix(1.0))
        .set_view_map();
    
    fg.show().unwrap();
    fg.echo_to_file("plot.gpl");
}

