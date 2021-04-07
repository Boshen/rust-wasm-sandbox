use gl_matrix::{common::Mat4, mat4, vec3};
use std::f32::consts::{FRAC_PI_4, PI, TAU};
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
        let canvas = dom::canvas("canvas");
        let gl = dom::canvas_context::<WebGlRenderingContext>(&canvas, "webgl");
        let cube_program = App::get_cube_program(&App::get_vertex_source(), &App::get_fragment_source())?;

        let sphere_objects = vec![Object {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        }];
        let sphere_program =
            App::get_sphere_program(&App::get_vertex_source(), &App::get_fragment_source(), sphere_objects)?;

        Ok(App {
            gl,
            programs: vec![cube_program, sphere_program],
        })
    }

    fn get_vertex_source() -> &'static str {
        r#"
        attribute vec4 a_position;
        attribute vec4 a_color;
        attribute vec3 a_normal;

        uniform mat4 u_model_view_matrix;
        uniform mat4 u_projection_matrix;
        uniform mat4 u_normal_matrix;

        varying vec4 v_color;
        varying vec3 v_normal;

        void main() {
          gl_Position = u_projection_matrix * u_model_view_matrix * a_position;
          v_color = a_color;
          v_normal = mat3(u_normal_matrix) * a_normal;
        }
      "#
    }

    fn get_fragment_source() -> &'static str {
        r#"
        precision mediump float;

        uniform vec3 u_light_direction;

        varying vec4 v_color;
        varying vec3 v_normal;

        void main() {
          float light = max(dot(v_normal, u_light_direction), 0.0);
          gl_FragColor = vec4(v_color.rgb * light, 1.0);
        }
    "#
    }

    fn get_sphere_program(
        vertex_source: &str,
        fragment_source: &str,
        objects: Vec<Object>,
    ) -> Result<Program, JsValue> {
        let sphere = Sphere::new(1.0, 128, 128, 0.0, TAU, 0.0, TAU);
        let sphere_colors = sphere.indices.iter().map(|_| 1.0).collect();
        Program::new(
            "canvas",
            ProgramDescription {
                vertex_source,
                fragment_source,
                objects,
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
                    Attribute {
                        name: "a_normal",
                        attribute_type: AttributeType::Vector(Dimension::D3),
                        vertices: sphere.normals,
                    },
                ],
            },
        )
    }

    fn get_cube_program(vertex_source: &str, fragment_source: &str) -> Result<Program, JsValue> {
        let cube = Cube::new(1, 1, 1);
        let cube_colors = [1.0; 96];
        Program::new(
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
                    Attribute {
                        name: "a_normal",
                        attribute_type: AttributeType::Vector(Dimension::D3),
                        vertices: cube.normals,
                    },
                ],
                objects: vec![
                    Object {
                        translation: [2.0, -2.0, 2.0],
                        rotation: [1.0, 0.0, 0.0],
                    },
                    Object {
                        translation: [-2.0, 2.0, 2.0],
                        rotation: [2.0, 0.0, 0.0],
                    },
                ],
            },
        )
    }

    fn get_light_direction(&self) -> UniformValue {
        let mut light_direction = [0.0; 3];
        vec3::normalize(&mut light_direction, &[1.0, 1.0, 0.0]);
        UniformValue::Vector3(light_direction)
    }

    fn get_model_view_matrix(&self, object: &Object) -> (UniformValue, UniformValue) {
        let mut model_matrix = mat4::create();
        let mut translation_matrix = mat4::create();
        let mut x_rotation_matrix = mat4::create();
        let mut y_rotation_matrix = mat4::create();
        let mut z_rotation_matrix = mat4::create();

        mat4::translate(&mut translation_matrix, &mat4::create(), &object.translation);
        mat4::rotate_x(&mut x_rotation_matrix, &translation_matrix, object.rotation[0]);
        mat4::rotate_y(&mut y_rotation_matrix, &x_rotation_matrix, object.rotation[1]);
        mat4::rotate_z(&mut z_rotation_matrix, &y_rotation_matrix, object.rotation[2]);
        mat4::scale(&mut model_matrix, &z_rotation_matrix, &[1.0; 3]);

        let mut camera_matrix = mat4::create();
        let mut view_matrix = mat4::create();
        mat4::look_at(&mut camera_matrix, &[0., 0., -8.], &[0., 0., 0.], &[0., 1., 0.]);
        mat4::invert(&mut view_matrix, &camera_matrix);

        let mut model_view_matrix = mat4::create();
        mat4::mul(&mut model_view_matrix, &view_matrix, &model_matrix);

        let normal_matrix = self.get_normal_matrix(&model_view_matrix);
        (UniformValue::Matrix4(model_view_matrix), normal_matrix)
    }

    fn get_projection_matrix(&self, aspect: f32) -> UniformValue {
        let fov = 45.0 * PI / 180.0;
        let z_near = 1.0;
        let z_far = 2000.0;
        let mut project_matrix: Mat4 = [0.0; 16];
        mat4::perspective(&mut project_matrix, fov, aspect, z_near, Some(z_far));
        UniformValue::Matrix4(project_matrix)
    }

    fn get_normal_matrix(&self, model_view_matrix: &Mat4) -> UniformValue {
        let mut invert_matrix = mat4::create();
        let mut normal_matrix = mat4::create();
        mat4::invert(&mut invert_matrix, &model_view_matrix);
        mat4::transpose(&mut normal_matrix, &invert_matrix);
        UniformValue::Matrix4(normal_matrix)
    }

    pub fn update(&mut self, t: f32) {
        self.programs.iter_mut().for_each(|p| {
            p.objects.iter_mut().for_each(|o| {
                o.rotation[0] = 0.0;
                o.rotation[1] = t * FRAC_PI_4;
                o.rotation[2] = 0.0;
            })
        })
    }

    pub fn render(&self) {
        Program::clear_gl(&self.gl);
        self.programs.iter().for_each(|p| {
            p.prepare_render();
            p.objects.iter().for_each(|o| {
                p.set_uniform("u_light_direction", self.get_light_direction());
                let (model_view_matrix, normal_matrix) = self.get_model_view_matrix(&o);
                p.set_uniform("u_model_view_matrix", model_view_matrix);
                p.set_uniform("u_normal_matrix", normal_matrix);
                let aspect = p.gl.drawing_buffer_width() as f32 / p.gl.drawing_buffer_height() as f32;
                p.set_uniform("u_projection_matrix", self.get_projection_matrix(aspect));
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
