use core::f32::consts::PI;

pub struct Sphere {
    pub indices: Vec<u16>,
    pub vertices: Vec<f32>,
}
// ( radius = 1, width_segments = 8, height_segments = 6, phi_start = 0, phi_length = Math.PI * 2, theta_start = 0, theta_length = Math.PI ) {

impl Sphere {
    pub fn new(
        radius: f32,
        width_segments: i32,
        height_segments: i32,
        phi_start: f32,
        phi_length: f32,
        theta_start: f32,
        theta_length: f32,
    ) -> Sphere {
        let width_segments = width_segments.max(3);
        let height_segments = height_segments.max(2);
        let theta_end = PI.min(theta_start + theta_length);

        let mut index = 0;
        let mut grid = vec![] as Vec<Vec<u16>>;
        let mut vertex = (0.0, 0.0, 0.0);
        let mut indices = vec![] as Vec<u16>;
        let mut vertices = vec![] as Vec<f32>;

        for iy in 0..(height_segments + 1) {
            let mut vertices_row = vec![];
            let v = iy as f32 / height_segments as f32;
            for ix in 0..(width_segments + 1) {
                let u = ix as f32 / width_segments as f32;
                vertex.0 = -radius * (phi_start + u * phi_length).cos() * (theta_start + v * theta_length).sin();
                vertex.1 = radius * (theta_start + v * theta_length).cos();
                vertex.2 = radius * (phi_start + u * phi_length).sin() * (theta_start + v * theta_length).sin();

                vertices.push(vertex.0);
                vertices.push(vertex.1);
                vertices.push(vertex.2);

                vertices_row.push(index);
                index += 1;
            }
            grid.push(vertices_row);
        }

        for iy in 0..height_segments {
            for ix in 0..width_segments {
                let a = grid[iy as usize][(ix + 1) as usize];
                let b = grid[iy as usize][ix as usize];
                let c = grid[(iy + 1) as usize][ix as usize];
                let d = grid[(iy + 1) as usize][(ix + 1) as usize];

                if iy != 0 || theta_start > 0.0 {
                    indices.push(a);
                    indices.push(b);
                    indices.push(d);
                    web_sys::console::log_3(&a.into(), &b.into(), &c.into());
                }
                if iy != height_segments - 1 || theta_end < PI {
                    indices.push(b);
                    indices.push(c);
                    indices.push(d);
                    web_sys::console::log_3(&b.into(), &c.into(), &d.into());
                }
            }
        }

        Sphere { vertices, indices }
    }
}
