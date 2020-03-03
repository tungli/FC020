use ode_solvers::dop853::*;
use ode_solvers::*;

use std::path::Path;
use std::io::Write;
use std::fs::File;

type State = VectorN<f64, typenum::consts::U200>;
type Time = f64;

struct DiffRecomb {
  D: f64,
  alpha: f64,
  x_max: f64,
  N: usize
}

impl DiffRecomb {
    pub fn dx(&self) -> f64 {
        self.x_max/self.N as f64 as f64
    }

    pub fn init_state<F>(&self, f: F) -> State where F: Fn(f64) -> f64
    {
        State::from_iterator((0..self.N).map(|i| i as f64*self.x_max/self.N as f64).map(f))
    }
}

impl ode_solvers::System<State> for DiffRecomb {
    //RHS of ODEs
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let k = self.D/self.dx()/self.dx();
        dy[0] = k*2.0*(y[1] - y[0]) - self.alpha*y[0]*y[0];
        for i in 1..(self.N-1) {
            dy[i] = k*(y[i-1] - 2.0*y[i] + y[i+1]) - self.alpha*y[i]*y[i];
        }
        dy[self.N-1] = k*(y[self.N-2] - 2.0*y[self.N-1]) - self.alpha*y[self.N-1]*y[self.N-1];
    }
}

pub fn save(times: &Vec<Time>, states: &Vec<State>, filename: &Path) {
    // Create or open file
    let mut buf = match File::create(filename) {
        Err(e) => {
            println!("Could not open file. Error: {:?}", e);
            return;
        }
        Ok(buf) => buf,
    };

    // Write time and state vector in a csv format
    for (i, state) in states.iter().enumerate() {
        buf.write_fmt(format_args!("{}", times[i])).unwrap();
        for val in state.iter() {
            buf.write_fmt(format_args!(", {}", val)).unwrap();
        }
        buf.write_fmt(format_args!("\n")).unwrap();
    }
}

fn main() {
    // all in cm, otherwise SI.
    let size = 200;
    let t_final = 9e-4;
    let dt = t_final/1000.0;
    let system = DiffRecomb {
        alpha: 2e-7,
        D: 2000.0,
        N: size,
        x_max: 25.0
    };

    println!("Courant No.: {}", system.D*dt/system.dx()/system.dx());

    let y0: State = system.init_state(|x| if x > 10.0 { 0.0 } else { 2e11 });

    let mut stepper = Dop853::new(system, 0.0, t_final, dt, y0, 1.0e-10, 1.0e-10);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("{}", stats);

            let path = Path::new("data.dat");
            save(stepper.x_out(), stepper.y_out(), path);
            println!("Results saved in: {:?}", path);
        },
        Err(e) => println!("An error occured: {}", e),
    }
}

