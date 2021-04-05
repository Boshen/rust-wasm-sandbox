pub struct Cube {
    pub indices: Vec<u16>,
    pub vertices: Vec<f32>,
    pub normals: Vec<f32>,
}

impl Cube {
    #[rustfmt::skip]
    pub fn new(w: i32, h: i32, d: i32) -> Cube {
        let mut indices = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut ns = 0;
        let ws = 1; // width segment
        let hs = 1; // height segment
        let ds = 1; // depth segment

        Cube::build(&mut indices, &mut vertices, &mut normals, &mut ns, 2, 1, 0, -1, -1, d, h, w, ds, hs); // px
        Cube::build(&mut indices, &mut vertices, &mut normals, &mut ns, 2, 1, 0, 1, -1, d, h, -w, ds, hs); // nx
        Cube::build(&mut indices, &mut vertices, &mut normals, &mut ns, 0, 2, 1, 1, 1, w, d, h, ws, ds); // py
        Cube::build(&mut indices, &mut vertices, &mut normals, &mut ns, 0, 2, 1, 1, -1, w, d, -h, ws, ds); // ny
        Cube::build(&mut indices, &mut vertices, &mut normals, &mut ns, 0, 1, 2, 1, -1, w, h, d, ws, hs); // pz
        Cube::build(&mut indices, &mut vertices, &mut normals, &mut ns, 0, 1, 2, -1, -1, w, h, -d, ws, hs); // nz
        Cube {
            indices,
            vertices,
            normals,
        }
    }

    fn build(
        indices: &mut Vec<u16>,
        vertices: &mut Vec<f32>,
        normals: &mut Vec<f32>,
        num_of_vertices: &mut i32,
        u: usize,
        v: usize,
        w: usize,
        udir: i32,
        vdir: i32,
        width: i32,
        height: i32,
        depth: i32,
        grid_x: i32,
        grid_y: i32,
    ) {
        let segment_width = width as f32 / grid_x as f32;
        let segment_height = height as f32 / grid_y as f32;
        let width_half = width as f32 / 2.0;
        let height_half = height as f32 / 2.0;
        let depth_half = depth as f32 / 2.0;
        let grid_x1 = grid_x + 1;
        let grid_y1 = grid_y + 1;
        let mut vertex_counter = 0;
        let mut vector = [0.0; 3];

        for iy in 0..grid_y1 {
            let y = iy as f32 * segment_height - height_half;
            for ix in 0..grid_x1 {
                let x = ix as f32 * segment_width - width_half;
                vector[u] = x * udir as f32;
                vector[v] = y * vdir as f32;
                vector[w] = depth_half;
                vertices.push(vector[0]);
                vertices.push(vector[1]);
                vertices.push(vector[2]);
                vector[u] = 0.0;
                vector[v] = 0.0;
                vector[w] = if depth > 0 { 1.0 } else { -1.0 };
                normals.push(vector[0]);
                normals.push(vector[1]);
                normals.push(vector[2]);
                vertex_counter += 1;
            }
        }

        for iy in 0..grid_y {
            for ix in 0..grid_x {
                let a = *num_of_vertices + ix + grid_x1 * iy;
                let b = *num_of_vertices + ix + grid_x1 * (iy + 1);
                let c = *num_of_vertices + (ix + 1) + grid_x1 * (iy + 1);
                let d = *num_of_vertices + (ix + 1) + grid_x1 * iy;
                indices.push(a as u16);
                indices.push(b as u16);
                indices.push(d as u16);
                indices.push(b as u16);
                indices.push(c as u16);
                indices.push(d as u16);
            }
        }

        *num_of_vertices += vertex_counter;
    }
}
