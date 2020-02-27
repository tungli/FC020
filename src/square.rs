use nalgebra::{DMatrix, DVector};
use itertools::Itertools;

#[derive(Debug)]
pub struct Linspace {
    from: f64,
    to: f64,
    n: usize
}


impl Linspace {
    pub fn new(from: f64, to: f64, n: usize) -> Linspace {
        Linspace { from: from, to: to, n: n }
    }

    fn dx(&self) -> f64 {
        (self.to - self.from)/(self.n - 1) as f64
    }

    pub fn gen(&self) -> impl Iterator<Item = f64> + '_ {
        (0..(self.n))
            .map(move |i| self.from + self.dx()*i as f64)
    }
}

#[derive(Debug)]
pub struct Square {
    l_corner: (f64, f64),
    length: f64,
    n: usize,
    pub domain: DMatrix<f64>
}



impl Square {
    pub fn new(a: (f64, f64), b: f64, n: usize) -> Square {
        Square {l_corner: a, length: b, n: n, domain: DMatrix::zeros(n, n)}
    }

//    pub fn sub2ind(&self, sub: (usize, usize)) -> usize {
//        sub.0 * self.n + sub.1
//    }

    pub fn sub2ind_interior(&self, sub: (usize, usize)) -> usize {
        ((sub.0 as isize - 1) * (self.n as isize - 2) + (sub.1 as isize - 1)) as usize
    }

//    pub fn ind2sub(&self, ind: usize) -> (usize, usize) {
//        (ind % self.n, ind / self.n)
//    }

    pub fn is_boundary(&self, (i, j): (usize, usize)) -> bool {
        i == 0 || j == 0 || i + 1 == self.n || j + 1 == self.n
    }

    pub fn x(&self) -> Vec<f64> {
        let x0 = self.l_corner.0;
        let x1 = x0 + self.length;
        Linspace::new(x0, x1, self.n)
            .gen()
            .collect()
    }

    pub fn y(&self) -> Vec<f64> {
        let y0 = self.l_corner.1;
        let y1 = y0 + self.length;
        Linspace::new(y0, y1, self.n)
            .gen()
            .collect()
    }

    pub fn set_left_bc_dirichlet<F>(&mut self, f: F) where F: FnMut(&f64) -> f64 {
        self.set_bc_dirichlet_generic(f, 0);
    }

    pub fn set_right_bc_dirichlet<F>(&mut self, f: F) where F: FnMut(&f64) -> f64 {
        self.set_bc_dirichlet_generic(f, 2);
    }

    pub fn set_upper_bc_dirichlet<F>(&mut self, f: F) where F: FnMut(&f64) -> f64 {
        self.set_bc_dirichlet_generic(f, 1);
    }

    pub fn set_lower_bc_dirichlet<F>(&mut self, f: F) where F: FnMut(&f64) -> f64 {
        self.set_bc_dirichlet_generic(f, 3);
    }

    fn set_bc_dirichlet_generic<F>(&mut self, f: F, wall: u32) where F: FnMut(&f64) -> f64 {
        // 0 for left, 1 for upper, 2 for right, 3 for lower
        let points = if wall == 0 || wall == 2 {
            self.y()
        } else {
            self.x()
        };

        let vals = DVector::from_vec(points.iter().map(f).collect());

        if wall == 0 {
            self.domain.column_mut(0).copy_from(&vals);
        }
        if wall == 1 {
            self.domain.row_mut(0).copy_from(&vals.transpose());
        }
        if wall == 2 {
            self.domain.column_mut(self.n-1).copy_from(&vals);
        }
        if wall == 3 {
            self.domain.row_mut(self.n-1).copy_from(&vals.transpose());
        }

    }

//    pub fn grid(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
//        (0..self.n).cartesian_product(0..self.n)
//    }

    pub fn laplace(&self) -> (DMatrix<f64>, DVector<f64>) {
        let stencil: [(isize, isize); 5] = [(0,0), (1,0), (-1, 0), (0, 1), (0, -1)];
        let stencil_vals: [f64; 5] = [4.0, -1.0, -1.0, -1.0, -1.0];

        let size = (self.n - 2)*(self.n - 2);
        let mut m = DMatrix::zeros(size, size);
        let mut rhs = DVector::zeros(size);

        for ix in 1..(self.n - 1) {
            for iy in 1..(self.n - 1) {
                let k = self.sub2ind_interior((ix, iy));
                for ((i, j), v) in stencil.iter()
                    .map(|x| (ix as isize + x.0, iy as isize + x.1))
                    .map(|x| (x.0 as usize, x.1 as usize))
                    .zip(stencil_vals.iter())
                    {
                        if self.is_boundary((i, j)) {
                            rhs[k] += - v*self.domain[(i, j)]
                        } else {
                            let l = self.sub2ind_interior((i, j));
                            m[(k, l)] = v.clone();
                        }
                    }
            }
        }
        (m, rhs)
    }
}


