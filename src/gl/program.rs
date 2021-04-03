use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::dom;
use crate::gl::{Attribute, AttributeLocation, UniformValue};

pub struct ProgramDescription<'a> {
    pub vertex_source: &'a str,
    pub fragment_source: &'a str,
    pub indices: Option<Vec<u16>>,
    pub attributes: Vec<Attribute>,
}

pub struct Program {
    pub gl: WebGlRenderingContext,
    pub program: WebGlProgram,
    pub indices_buffer: Option<(WebGlBuffer, i32)>,
    pub attributes: Vec<AttributeLocation>,
}

impl Program {
    pub fn new(canvas_id: &'static str, desc: ProgramDescription) -> Result<Program, JsValue> {
        let gl = Program::init_gl(canvas_id)?;
        let program = Program::create_program(&gl, desc.vertex_source, desc.fragment_source)?;
        let attributes = Program::init_attributes(&gl, &program, &desc.attributes);
        let indices_buffer = Program::init_indices_buffer(&gl, &desc.indices);

        Ok(Program {
            gl,
            program,
            indices_buffer,
            attributes,
        })
    }

    pub fn draw(&self) {
        if let Some((_buffer, n)) = self.indices_buffer.as_ref() {
            self.gl.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                *n,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        } else {
            self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
        }
    }

    pub fn set_attributes(&self) {
        self.attributes.iter().for_each(|attribute| {
            self.gl
                .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&attribute.buffer));
            self.gl.vertex_attrib_pointer_with_i32(
                attribute.location,
                attribute.num_of_components(),
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );
            self.gl.enable_vertex_attrib_array(attribute.location);
        });
        if let Some((indices_buffer, _)) = self.indices_buffer.as_ref() {
            self.gl
                .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(indices_buffer));
        }
    }

    pub fn set_uniform(&self, name: &str, value: UniformValue) {
        let location = self.gl.get_uniform_location(&self.program, name);
        match value {
            UniformValue::Int(x) => self.gl.uniform1i(location.as_ref(), x),
            UniformValue::IVector2([x, y]) => self.gl.uniform2i(location.as_ref(), x, y),
            UniformValue::IVector3([x, y, z]) => self.gl.uniform3i(location.as_ref(), x, y, z),
            UniformValue::IVector4([x, y, z, w]) => self.gl.uniform4i(location.as_ref(), x, y, z, w),
            UniformValue::Float(x) => self.gl.uniform1f(location.as_ref(), x),
            UniformValue::Vector2([x, y]) => self.gl.uniform2f(location.as_ref(), x, y),
            UniformValue::Vector3([x, y, z]) => self.gl.uniform3f(location.as_ref(), x, y, z),
            UniformValue::Vector4([x, y, z, w]) => self.gl.uniform4f(location.as_ref(), x, y, z, w),
            UniformValue::Matrix2(mat) => self.gl.uniform_matrix2fv_with_f32_array(location.as_ref(), false, &mat),
            UniformValue::Matrix3(mat) => self.gl.uniform_matrix3fv_with_f32_array(location.as_ref(), false, &mat),
            UniformValue::Matrix4(mat) => self.gl.uniform_matrix4fv_with_f32_array(location.as_ref(), false, &mat),
        }
    }

    pub fn clear_gl(&self) {
        self.gl
            .viewport(0, 0, self.gl.drawing_buffer_width(), self.gl.drawing_buffer_height());
        self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl
            .clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn init_indices_buffer(gl: &WebGlRenderingContext, indices: &Option<Vec<u16>>) -> Option<(WebGlBuffer, i32)> {
        indices.as_ref().map(|indices| {
            let buffer = gl.create_buffer();
            gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, buffer.as_ref());
            unsafe {
                gl.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    &js_sys::Uint16Array::view(indices),
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }
            (buffer.unwrap(), indices.len() as i32)
        })
    }

    pub fn init_attributes(
        gl: &WebGlRenderingContext,
        program: &WebGlProgram,
        attributes: &Vec<Attribute>,
    ) -> Vec<AttributeLocation> {
        attributes
            .iter()
            .map(|attribute| {
                let buffer = gl.create_buffer();
                gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, buffer.as_ref());
                unsafe {
                    gl.buffer_data_with_array_buffer_view(
                        WebGlRenderingContext::ARRAY_BUFFER,
                        &js_sys::Float32Array::view(&attribute.vertices),
                        WebGlRenderingContext::STATIC_DRAW,
                    );
                }

                AttributeLocation {
                    location: gl.get_attrib_location(program, attribute.name) as u32,
                    attribute_type: attribute.attribute_type,
                    buffer: buffer.unwrap(),
                }
            })
            .collect()
    }

    pub fn init_gl(canvas_id: &'static str) -> Result<WebGlRenderingContext, JsValue> {
        let canvas = dom::canvas(canvas_id);
        let gl = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        gl.enable(WebGlRenderingContext::CULL_FACE);
        gl.enable(WebGlRenderingContext::DEPTH_TEST | WebGlRenderingContext::DEPTH_BUFFER_BIT);
        gl.front_face(WebGlRenderingContext::CCW);
        gl.cull_face(WebGlRenderingContext::BACK);
        gl.depth_func(WebGlRenderingContext::LEQUAL);

        dom::resize_canvas_to_window_size(canvas_id);

        Ok(gl)
    }

    pub fn create_program(
        gl: &WebGlRenderingContext,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<WebGlProgram, String> {
        Program::link_program(
            gl,
            &Program::compile_shader(gl, WebGlRenderingContext::VERTEX_SHADER, vertex_source)?,
            &Program::compile_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, fragment_source)?,
        )
    }

    pub fn compile_shader(gl: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn link_program(
        gl: &WebGlRenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        gl.attach_shader(&program, vert_shader);
        gl.attach_shader(&program, frag_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
