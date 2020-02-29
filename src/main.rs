mod square;
mod cg_sparse;

use square::*;
use cg_sparse::*;

use std::fs::File;
use std::io::prelude::*;

use gnuplot::*;

use std::env;


fn main() {

    let args: Vec<String> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();

    let mut domain = Square::new((0.0, 0.0), 1.0, size);

    domain.set_upper_bc_dirichlet(|x| (-2.0*x).exp());
    domain.set_left_bc_dirichlet(|x| (2.0*x).cos());
    domain.set_lower_bc_dirichlet(|x| (-2.0*x).exp()*2.0_f64.cos());
    domain.set_right_bc_dirichlet(|x| (2.0*x).cos()*(-2.0_f64).exp());

    println!("BC set!");

    //print!("{}", domain.domain.to_string().as_str());

    let (m, bc_rhs) = domain.laplace();

    println!("Matrix assembled!");
    
    let res = conjugate_gradient(&m, &bc_rhs, 1e-8, Some(2000));

    println!("Matrix solved!");

    for i in 1..(size-1) {
        for j in 1..(size-1) {
            let k = domain.sub2ind_interior((i,j));
            domain.domain[(i,j)] = res[k];
        }
    }

    let s = domain.domain.to_string();
    let data = s.as_bytes();

    let mut buffer = File::create(format!("results_fd_laplace/solution_size_{}.mat", size))
        .expect("Writing solution problems");
    buffer.write_all(data)
        .expect("Solution write failed");

    let mut fg = Figure::new();

    let dx = domain.x()[1] - domain.x()[0];
    
    fg.axes3d()
    	.set_title(format!("Size: {}", size).as_str(), &[])
        .surface(domain.domain.transpose().iter(), size, size, Some((0.0, 0.0, 1.0 + dx, 1.0 + dx)), &[])
    	.set_x_label("X", &[])
    	.set_y_label("Y", &[])
    	.set_z_label("Z", &[])
        .set_x_range(Fix(0.0), Fix(1.0))
        .set_y_range(Fix(0.0), Fix(1.0))
        .set_view_map();
    
    fg.save_to_png(format!("results_fd_laplace/map_size_{}.png", size).as_str(), 600, 600)
        .expect("Could not save image!");
    fg.show().unwrap();
}

