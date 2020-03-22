use lsode::{solve_ode, linspace};
use std::fs::File;
use std::io::Write;


fn init_state<F>(n: usize, x_max: f64, f: F) -> Vec<f64> where F: Fn(f64) -> f64
{
    let y0: Vec<f64> = (0..n).map(|i| i as f64*x_max/n as f64).map(f).collect();
    y0
}


fn main() -> std::io::Result<()> {
    // all in cm, otherwise SI.
    let size: usize = 200;
    let x_max: f64 = 25.0;

    let t_final = 5e-1;
    let n_times = 100;
    let alpha = 2e-7;
    let diff_coef = 2000.0;

    let x = linspace(0.0, x_max, size);

    let derivs = |y: &[f64], _t: &f64| {
        let n = size;
        let dx = x_max/n as f64;
        let k = diff_coef/dx/dx;
    
        let mut dy: Vec<f64> = (0..n).map(|_| 0.0).collect();
    
        dy[0] = k*6.0*(y[1] - y[0]) - alpha*y[0]*y[0];
        for i in 1..(n-1) {
            dy[i] = diff_coef/dx/x[i]*(y[i+1] - y[i-1]) + 
                k*(y[i-1] - 2.0*y[i] + y[i+1]) - alpha*y[i]*y[i];
        }
        dy[n-1] = diff_coef/dx/x[n-1]*y[n-2] + k*(y[n-2] - 2.0*y[n-1]) - alpha*y[n-1]*y[n-1];
        dy
    };

    let ts: Vec<f64> = linspace(-7.0, 0.0, n_times).iter().map(|x| t_final * 2.0_f64.powf(*x)).collect();
    let y0 = init_state(size, x_max, |x| if x > 10.0 { 0.0 } else { 2e11 });

    let res = solve_ode(derivs, &y0, ts.clone(), 1e-3, 1e-3);


    for (i, t) in ts.iter().enumerate() {
        let filename = format!("../results_mol_diffusion/results_{}.out", i);
        println!("i: {} -> {}", i, t);
        let mut buf = File::create(filename)?;

        write!(buf, "{} {}\n", t, t)?;
        for (y, x) in res[i].iter().zip(linspace(0.0, x_max, size)) {
            write!(buf, "{} {}\n", x, y)?;
        }
    }
    Ok(())
}

