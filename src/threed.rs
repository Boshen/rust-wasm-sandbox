use gl_matrix::{common::Mat4, mat4};
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::geometry::{Cube, Sphere};
use crate::gl::{Attribute, AttributeType, Dimension, Object, Program, ProgramDescription, UniformValue};

struct App {
    gl: WebGlRenderingContext,
    programs: Vec<Program>,
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
          v_color = vec4(a_color.xyz, 1.0);
        }
    "#;
        let fragment_source = r#"
        precision mediump float;
        varying vec4 v_color;
        void main() {
          gl_FragColor = v_color;
        }
    "#;

        let sphere = Sphere::new(1.0, 32, 32, 0.0, TAU, 0.0, TAU);
        let sphere_colors = sphere.indices.iter().map(|_| 1.0).collect();
        let sphere_program = Program::new(
            "canvas",
            ProgramDescription {
                vertex_source,
                fragment_source,
                indices: Some(sphere.indices),
                attributes: vec![
                    Attribute {
                        name: "a_position",
                        attribute_type: AttributeType::Vector(Dimension::D3),
                        vertices: sphere.vertices,
                    },
                    Attribute {
                        name: "a_color",
                        attribute_type: AttributeType::Vector(Dimension::D4),
                        vertices: sphere_colors,
                    },
                ],
                objects: vec![Object {
                    translation: [0.0, 0.0, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                }],
            },
        )?;

        let cube = Cube::new(1, 1, 1);
        let cube_colors = [1.0; 96];
        let cube_program = Program::new(
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
                        vertices: cube_colors.to_vec(),
                    },
                ],
                objects: vec![
                    Object {
                        translation: [1.0, 1.0, 1.0],
                        rotation: [1.0, 0.0, 0.0],
                    },
                    Object {
                        translation: [2.0, 2.0, 2.0],
                        rotation: [2.0, 0.0, 0.0],
                    },
                ],
            },
        )?;

        let canvas = dom::canvas("canvas");
        let gl = dom::canvas_context::<WebGlRenderingContext>(&canvas, "webgl");
        Ok(App {
            gl,
            programs: vec![cube_program, sphere_program],
        })
    }

    fn get_world_matrix(&self, object: &Object) -> UniformValue {
        let mut world_matrix: Mat4 = [0.0; 16];
        let mut translation_matrix: Mat4 = [0.; 16];
        let mut x_rotation_matrix: Mat4 = [0.; 16];
        let mut y_rotation_matrix: Mat4 = [0.; 16];
        mat4::translate(&mut translation_matrix, &mat4::create(), &object.translation);
        mat4::rotate_x(&mut x_rotation_matrix, &translation_matrix, object.rotation[0]);
        mat4::rotate_y(&mut y_rotation_matrix, &x_rotation_matrix, object.rotation[1]);
        mat4::rotate_z(&mut world_matrix, &x_rotation_matrix, object.rotation[2]);
        UniformValue::Matrix4(world_matrix)
    }

    fn get_model_view_matrix(&self) -> UniformValue {
        let mut model_view_matrix: Mat4 = [0.0; 16];
        mat4::look_at(&mut model_view_matrix, &[0., 0., -8.], &[0., 0., 0.], &[0., 1., 0.]);
        UniformValue::Matrix4(model_view_matrix)
    }

    fn get_projection_matrix(&self, aspect: f32) -> UniformValue {
        let fov = 45.0 * PI / 180.0;
        let z_near = 0.1;
        let z_far = 100.0;
        let mut project_matrix: Mat4 = [0.0; 16];
        mat4::perspective(&mut project_matrix, fov, aspect, z_near, Some(z_far));
        UniformValue::Matrix4(project_matrix)
    }

    pub fn update(&mut self, t: f32) {
        self.programs.iter_mut().for_each(|p| {
            p.objects.iter_mut().for_each(|o| {
                o.rotation[0] = t * FRAC_PI_2;
                o.rotation[1] = t * PI;
                o.rotation[2] = t * PI;
            })
        })
    }

    pub fn render(&self) {
        Program::clear_gl(&self.gl);
        self.programs.iter().for_each(|p| {
            p.prepare_render();
            p.objects.iter().for_each(|o| {
                p.set_uniform("u_world_matrix", self.get_world_matrix(&o));
                p.set_uniform("u_model_view_matrix", self.get_model_view_matrix());
                p.set_uniform(
                    "u_projection_matrix",
                    self.get_projection_matrix(
                        p.gl.drawing_buffer_width() as f32 / p.gl.drawing_buffer_height() as f32,
                    ),
                );
                p.render();
            })
        })
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn threed() -> Result<(), JsValue> {
    let app = App::new()?;
    let app = Rc::new(RefCell::new(app));
    dom::request_animation_frame(move |t, _dt| {
        app.borrow_mut().update(t);
        app.borrow().render();
    });
    Ok(())
}
