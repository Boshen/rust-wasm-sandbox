use gl_matrix::{common::Mat4, mat4};
use std::rc::Rc;
use std::{cell::RefCell, f32::consts::PI};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::geometry::Cube;
use crate::gl::{Attribute, AttributeType, Dimension, Program, ProgramDescription, UniformValue};

struct App {
    program: Program,
}

use crate::dom;

impl App {
    pub fn new() -> Result<App, JsValue> {
        let vertex_source = r#"
        attribute vec4 a_position;
        attribute vec4 a_color;
        uniform mat4 u_world_matrix;
        uniform mat4 u_model_view_matrix;
        uniform mat4 u_projection_matrix;
        varying vec4 v_color;

        void main() {
          gl_Position = u_projection_matrix * u_model_view_matrix * u_world_matrix * a_position;
          v_color = a_color;
        }
    "#;
        let fragment_source = r#"
        precision mediump float;
        varying vec4 v_color;
        void main() {
          gl_FragColor = v_color;
        }
    "#;

        let cube = Cube::new(1, 1, 1);
        let program = Program::new(
            "canvas",
            ProgramDescription {
                vertex_source,
                fragment_source,
                indices: Some(cube.indices),
                attributes: vec![
                    Attribute {
                        name: "a_position",
                        attribute_type: AttributeType::Vector(Dimension::D3),
                        vertices: cube.vertices,
                    },
                    Attribute {
                        name: "a_color",
                        attribute_type: AttributeType::Vector(Dimension::D4),
                        vertices: App::cube_colors(),
                    },
                ],
            },
        )?;

        Ok(App { program })
    }

    #[rustfmt::skip]
    fn cube_colors() -> Vec<f32>{
        let face_colors = vec![
            vec![0.0,  0.0,  0.0,  1.0],    // Front face: black
            vec![1.0,  0.0,  0.0,  1.0],    // Back face: red
            vec![0.0,  1.0,  0.0,  1.0],    // Top face: green
            vec![0.0,  0.0,  1.0,  1.0],    // Bottom face: blue
            vec![1.0,  1.0,  0.0,  1.0],    // Right face: yellow
            vec![1.0,  0.0,  1.0,  1.0],    // Left face: purple
        ];
        face_colors.iter().flat_map(|c| c.repeat(4)).collect()
    }

    fn set_rotation(&self, t: f32) {
        let mut world_matrix: Mat4 = [0.0; 16];
        mat4::identity(&mut world_matrix);

        let mut identity_matrix: Mat4 = [0.; 16];
        mat4::identity(&mut identity_matrix);

        let mut x_rotation_matrix: Mat4 = [0.; 16];
        let mut y_rotation_matrix: Mat4 = [0.; 16];
        let angle: f32 = t / 2.0 * PI;

        mat4::rotate(&mut y_rotation_matrix, &identity_matrix, angle, &[0., 1., 0.]);
        mat4::rotate(&mut x_rotation_matrix, &identity_matrix, angle / 4.0, &[1., 0., 0.]);
        mat4::mul(&mut world_matrix, &y_rotation_matrix, &x_rotation_matrix);

        self.program
            .set_uniform("u_world_matrix", UniformValue::Matrix4(world_matrix));
    }

    pub fn render(&self, t: f32) {
        self.program.clear_gl();
        self.program.gl.use_program(Some(&self.program.program));

        self.program.set_attributes();

        let fov = 45.0 * PI / 180.0;
        let aspect = self.program.gl.drawing_buffer_width() as f32 / self.program.gl.drawing_buffer_height() as f32;
        let z_near = 0.1;
        let z_far = 100.0;
        let mut project_matrix: Mat4 = [0.0; 16];
        let mut model_view_matrix: Mat4 = [0.0; 16];

        mat4::look_at(&mut model_view_matrix, &[0., 0., -8.], &[0., 0., 0.], &[0., 1., 0.]);
        mat4::perspective(&mut project_matrix, fov, aspect, z_near, Some(z_far));

        self.set_rotation(t);

        self.program
            .set_uniform("u_projection_matrix", UniformValue::Matrix4(project_matrix));

        self.program
            .set_uniform("u_model_view_matrix", UniformValue::Matrix4(model_view_matrix));

        self.program.gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            36,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn threed() -> Result<(), JsValue> {
    let app = App::new()?;
    let app = Rc::new(RefCell::new(app));
    dom::request_animation_frame(move |t, _dt| {
        app.borrow().render(t);
    });
    Ok(())
}
