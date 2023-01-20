pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

// pub struct FluidSolver2D {
//     nx: usize,
//     ny: usize,
//     dx: f32,
//     dt: f32,
//     n_iter: u32,

//     vx: Vec<f32>,
//     vy: Vec<f32>,
//     pressure: Vec<f32>,
//     div: Vec<f32>,
//     collider: Vec<bool>,
// }

// impl FluidSolver2D {
//     pub fn new(nx: usize, ny: usize, dx: f32, dt: f32, n_iter: u32) -> Self {
//         Self {
//             nx,
//             ny,
//             dx,
//             dt,
//             n_iter,
//             vx: vec![0f32; nx * ny],
//             vy: vec![0f32; nx * ny],
//             pressure: vec![0f32; nx * ny],
//             div: vec![0f32; nx * ny],
//             collider: vec![false; nx * ny],
//         }
//     }

//     pub fn interpolate(&self, pos: (f32, f32), field: Vec<f32>) -> f32 {
//         let i = clamp(pos.0 / self.dx, 1f32, (self.nx - 1) as f32) as u32;
//         let j = clamp(pos.1 / self.dx, 1f32, (self.ny - 1) as f32) as u32;

//         0f32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(3, 1, 5), 3);
        assert_eq!(clamp(3, 4, 5), 4);
        assert_eq!(clamp(3, 1, 2), 2);
        assert_eq!(clamp(3, 2, 2), 2);
    }

    #[test]
    #[should_panic]
    fn test_clamp_panic() {
        clamp(3, 5, 4);
    }
}
