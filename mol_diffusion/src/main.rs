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

    let t_final = 1e1;
    let n_times = 100000;
    let alpha = 2e-7;
    let diff_coef = 2000.0;

    let x = linspace(0.0, x_max, size);

    let derivs = |y: &[f64], _t: &f64| {
        let n = size;
        let dx = x_max/n as f64;
        let k = diff_coef/dx/dx;
    
        let mut dy: Vec<f64> = (0..n).map(|_| 0.0).collect();
    
        dy[0] = k*2.0*(y[1] - y[0]) - alpha*y[0]*y[0];
        for i in 1..(n-1) {
            dy[i] = diff_coef/dx/2.0/x[i]*(y[i+1] - y[i-1]) + 
                k*(y[i-1] - 2.0*y[i] + y[i+1]) - alpha*y[i]*y[i];
        }
        dy[n-1] = diff_coef/dx/2.0/x[n-1]*y[n-2] + k*(y[n-2] - 2.0*y[n-1]) - alpha*y[n-1]*y[n-1];
        dy
    };

    let ts = linspace(0.0, t_final, n_times);
    let y0 = init_state(size, x_max, |x| if x > 10.0 { 0.0 } else { 2e11 });

    let res = solve_ode(derivs, &y0, ts.clone(), 1e-3, 1e-3);


    let save_at: Vec<usize> = [1, 11, 1001, 10001, 100000].iter().map(|i| i-1 as usize).collect();

    for (i, t) in (0..save_at.len()).zip(save_at.iter().map(|j| ts[*j]).collect::<Vec<f64>>()) {
        let filename = format!("../results_mol_diffusion/results_{}.out", i);
        println!("i: {} -> {}", i, t);
        let mut buf = File::create(filename)?;

        write!(buf, "{} {}\n", t, t)?;
        for (y, x) in res[save_at[i]].iter().zip(linspace(0.0, x_max, size)) {
            write!(buf, "{} {}\n", x, y)?;
        }
    }
    Ok(())
}

