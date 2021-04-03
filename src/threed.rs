use gl_matrix::{common::Mat4, mat4};
use std::rc::Rc;
use std::{cell::RefCell, f32::consts::PI};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

struct App {
    gl: WebGlRenderingContext,
    program: WebGlProgram,

    position_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
}

use crate::dom;
use crate::webgl;

impl App {
    pub fn new() -> Result<App, JsValue> {
        let gl = webgl::init_gl("canvas")?;
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
        let frag_source = r#"
        precision mediump float;
        varying vec4 v_color;
        void main() {
          gl_FragColor = v_color;
        }
    "#;
        let program = webgl::create_program(&gl, &vertex_source, &frag_source)?;

        let position_buffer = webgl::create_buffer(&gl)?;
        webgl::bind_array_buffer(&gl, &position_buffer);
        webgl::buffer_data_f32(&gl, &webgl::cube_vertices());

        let index_buffer = webgl::create_buffer(&gl)?;
        webgl::bind_element_array_buffer(&gl, &index_buffer);
        unsafe {
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &js_sys::Uint16Array::view(&webgl::cube_indices()),
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let color_buffer = webgl::create_buffer(&gl)?;
        webgl::bind_array_buffer(&gl, &color_buffer);
        webgl::buffer_data_f32(&gl, &App::cube_colors());

        Ok(App {
            gl,
            program,
            position_buffer,
            index_buffer,
            color_buffer,
        })
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

    fn set_ratation(&self, t: f32) {
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

        self.gl.uniform_matrix4fv_with_f32_array(
            self.gl.get_uniform_location(&self.program, "u_world_matrix").as_ref(),
            false,
            &world_matrix,
        );
    }

    pub fn render(&self, t: f32) {
        webgl::clear_gl(&self.gl);
        self.gl.use_program(Some(&self.program));

        let attribute_position = self.gl.get_attrib_location(&self.program, "a_position") as u32;
        webgl::bind_array_buffer(&self.gl, &self.position_buffer);
        self.gl
            .vertex_attrib_pointer_with_i32(attribute_position, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(attribute_position);

        let attribute_color = self.gl.get_attrib_location(&self.program, "a_color") as u32;
        webgl::bind_array_buffer(&self.gl, &self.color_buffer);
        self.gl
            .vertex_attrib_pointer_with_i32(attribute_color, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(attribute_color);

        webgl::bind_element_array_buffer(&self.gl, &self.index_buffer);

        let fov = 45.0 * PI / 180.0;
        let aspect = self.gl.drawing_buffer_width() as f32 / self.gl.drawing_buffer_height() as f32;
        let z_near = 0.1;
        let z_far = 100.0;
        let mut project_matrix: Mat4 = [0.0; 16];
        let mut model_view_matrix: Mat4 = [0.0; 16];

        mat4::look_at(&mut model_view_matrix, &[0., 0., -8.], &[0., 0., 0.], &[0., 1., 0.]);
        mat4::perspective(&mut project_matrix, fov, aspect, z_near, Some(z_far));

        self.set_ratation(t);

        self.gl.uniform_matrix4fv_with_f32_array(
            self.gl
                .get_uniform_location(&self.program, "u_projection_matrix")
                .as_ref(),
            false,
            &project_matrix,
        );

        self.gl.uniform_matrix4fv_with_f32_array(
            self.gl
                .get_uniform_location(&self.program, "u_model_view_matrix")
                .as_ref(),
            false,
            &model_view_matrix,
        );

        self.gl.draw_elements_with_i32(
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
