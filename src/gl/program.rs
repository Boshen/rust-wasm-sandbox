use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::dom;
use crate::gl::{Attribute, AttributeLocation};

pub struct ProgramDescription<'a> {
    pub vertex_source: &'a str,
    pub fragment_source: &'a str,
    pub attributes: Vec<Attribute>,
}

pub struct Program {
    pub gl: WebGlRenderingContext,
    pub program: WebGlProgram,
    pub attributes: Vec<AttributeLocation>,
}

impl Program {
    pub fn new(canvas_id: &'static str, desc: ProgramDescription) -> Result<Program, JsValue> {
        let gl = Program::init_gl(canvas_id)?;
        let program = Program::create_program(&gl, desc.vertex_source, desc.fragment_source)?;
        let attributes = Program::init_attributes(&gl, &program, &desc.attributes);

        Ok(Program {
            gl,
            program,
            attributes,
        })
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
            if let Some(element_array_buffer) = attribute.element_array_buffer.as_ref() {
                self.gl
                    .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(element_array_buffer));
            }
        })
    }

    pub fn clear_gl(&self) {
        self.gl
            .viewport(0, 0, self.gl.drawing_buffer_width(), self.gl.drawing_buffer_height());
        self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl
            .clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
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

                let element_array_buffer = attribute.element_array.as_ref().map(|element_array| {
                    let element_array_buffer = gl.create_buffer();
                    gl.bind_buffer(
                        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                        element_array_buffer.as_ref(),
                    );
                    unsafe {
                        gl.buffer_data_with_array_buffer_view(
                            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                            &js_sys::Uint16Array::view(element_array),
                            WebGlRenderingContext::STATIC_DRAW,
                        );
                    }
                    element_array_buffer.unwrap()
                });

                AttributeLocation {
                    location: gl.get_attrib_location(program, attribute.name) as u32,
                    attribute_type: attribute.attribute_type,
                    buffer: buffer.unwrap(),
                    element_array_buffer,
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
