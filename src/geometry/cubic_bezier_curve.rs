pub struct CubicBezierCurve {
    pub vertices: Vec<f32>,
    pub number_of_vertices: i32,
}

impl CubicBezierCurve {
    pub fn new(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3], v3: [f32; 3]) -> CubicBezierCurve {
        let mut vertices = vec![];
        let count = 300;
        let step = 1.0 / count as f32;
        for i in 0..count {
            let t = i as f32 * step;
            let p0 = CubicBezierCurve::cubic_bezier(t, v0[0], v1[0], v2[0], v3[0]);
            let p1 = CubicBezierCurve::cubic_bezier(t, v0[1], v1[1], v2[1], v3[1]);
            let p2 = CubicBezierCurve::cubic_bezier(t, v0[2], v1[2], v2[2], v3[2]);
            vertices.push(p0);
            vertices.push(p1);
            vertices.push(p2);
        }
        CubicBezierCurve {
            vertices,
            number_of_vertices: count,
        }
    }

    fn cubic_bezier(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
        CubicBezierCurve::cubic_bezier_p0(t, p0)
            + CubicBezierCurve::cubic_bezier_p1(t, p1)
            + CubicBezierCurve::cubic_bezier_p2(t, p2)
            + CubicBezierCurve::cubic_bezier_p3(t, p3)
    }

    fn cubic_bezier_p0(t: f32, p: f32) -> f32 {
        let k = 1.0 - t;
        return k * k * k * p;
    }

    fn cubic_bezier_p1(t: f32, p: f32) -> f32 {
        let k = 1.0 - t;
        return 3.0 * k * k * t * p;
    }

    fn cubic_bezier_p2(t: f32, p: f32) -> f32 {
        return 3.0 * (1.0 - t) * t * t * p;
    }

    fn cubic_bezier_p3(t: f32, p: f32) -> f32 {
        return t * t * t * p;
    }
}
