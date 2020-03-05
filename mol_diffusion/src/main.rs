use lsode::solve_ode;

const N: usize = 200;
const X: f64 = 25.0;

fn init_state<F>(n: usize, x_max: f64, f: F) -> Vec<f64> where F: Fn(f64) -> f64
{
    let y0: Vec<f64> = (0..n).map(|i| i as f64*x_max/n as f64).map(f).collect();
    y0
}

fn derivs(y: &[f64], t: &f64) -> Vec<f64> {
    let alpha = 2e-7;
    let D = 2000.0;
    let n = N;
    let x_max = X;
    let dx = x_max/n as f64;
    let k = D/dx/dx;

    let mut dy: Vec<f64> = (0..n).map(|_| 0.0).collect();

    dy[0] = k*2.0*(y[1] - y[0]) - alpha*y[0]*y[0];
    for i in 1..(n-1) {
        dy[i] = k*(y[i-1] - 2.0*y[i] + y[i+1]) - alpha*y[i]*y[i];
    }
    dy[n-1] = k*(y[n-2] - 2.0*y[n-1]) - alpha*y[n-1]*y[n-1];
    dy
}

fn main() {
    // all in cm, otherwise SI.
    let size = N;
    let t_final = 1e0;
    let n_times = 50000;
    let x_max = X;

    let y0 = init_state(size, x_max, |x| if x > 10.0 { 0.0 } else { 2e11 });
    let ts: Vec<f64> = (0..n_times).map(|i| t_final/(n_times - 1) as f64 * i as f64).collect();

    let res = solve_ode(derivs, &y0, ts.clone(), 1e-6, 1e-6);
    println!("{:?}", res.last());

}

