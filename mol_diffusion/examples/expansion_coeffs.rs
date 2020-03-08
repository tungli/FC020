use gnuplot::*;
use lsode::linspace;
use nalgebra::{DVector, DMatrix};

const PI: f64 = 3.1415926535;

fn init_state<F>(n: usize, x_max: f64, f: F) -> Vec<f64> where F: Fn(f64) -> f64
{
    let y0: Vec<f64> = (0..n).map(|i| i as f64*x_max/n as f64).map(f).collect();
    y0
}

fn basis_fs(n: i32, x_max: f64, size: usize) -> Vec<f64> {
    let x = linspace(0.0, x_max, size);
    x.iter().map(|x| (n as f64*PI*x/2.0/x_max).cos()).collect()
}

fn product(dx: f64, f1: &Vec<f64>, f2: &Vec<f64>) -> f64 {
    f1.iter().zip(f2).map( |(y1, y2)| y1 * y2).sum::<f64>()*dx
}


fn main() {
    let size: usize = 2000;
    let x_max: f64 = 25.0;

    let x = linspace(0.0, x_max, size);
    let y0 = init_state(size, x_max, |x| if x > 10.0 { 0.0 } else { 2e11 });


    let norms: Vec<f64> = (0..10).map(|i| { 
        let b = basis_fs(i, x_max, size);
        let dx = x[1] - x[0];
        product(dx, &b, &b)
    }).collect();


    let mut m = DMatrix::identity(10, 10);

    for i in 1..10 {
        for j in 1..10 {
            m[(i-1, j-1)] = product(x[1] - x[0], &basis_fs(i as i32, x_max, size), &basis_fs(j as i32, x_max, size));
        }
    }
    let minv = m.try_inverse().expect("failed inversion");

    let projections: Vec<f64> = (1..11).map(|i| {
        let b = basis_fs(i, x_max, size);
        let dx = x[1] - x[0];
        product(dx, &y0, &b)
    }).collect();

    let p = DVector::<f64>::from_vec(projections);

    println!("{:?}", minv);
    println!("{:?}", p);

    let coeffs = minv * p;

    println!("{:?}", coeffs);

    let approx: Vec<f64> = (1..10).fold(vec![0.0_f64; x.len()], |mut acc, i| {
        let mut a = basis_fs(i as i32, x_max, size);
        for j in 0..a.len() {
            a[j] *= coeffs[i - 1];
        }

        for j in 0..x.len() {
            acc[j] += a[j];
        }
        acc
    });


    let mut fg = Figure::new();
    fg.axes2d()
    	.set_title("A plot", &[])
    	.set_legend(Graph(0.5), Graph(0.9), &[], &[])
    	.set_x_label("x", &[])
    	.set_y_label("u", &[])
    	.lines(
            x.clone(), y0,
    		&[Caption("Initial")],
    	)
    	.lines(
            x, approx,
    		&[Caption("Approx")],
    	);

    fg.show().unwrap();

}
