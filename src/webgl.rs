use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::dom;

pub struct Attribute {
    pub name: &'static str,
    pub buffer: WebGlBuffer,
    pub num_of_components: i32,
}

pub fn init_gl(id: &'static str) -> Result<WebGlRenderingContext, JsValue> {
    let canvas = dom::canvas(id);
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

    resize_canvas_to_window_size(id);
    Ok(gl)
}

pub fn resize_canvas_to_window_size(id: &'static str) {
    let closure = Closure::wrap(Box::new(move || {
        let c = dom::canvas(id);
        let client_width = c.client_width() as u32;
        let client_height = c.client_height() as u32;
        if c.width() != client_width || c.height() != client_height {
            c.set_width(client_width);
            c.set_height(client_height);
        }
    }) as Box<dyn FnMut()>);
    dom::window()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

pub fn create_program(
    gl: &WebGlRenderingContext,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<WebGlProgram, String> {
    link_program(
        gl,
        &compile_shader(gl, WebGlRenderingContext::VERTEX_SHADER, vertex_source)?,
        &compile_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, fragment_source)?,
    )
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

pub fn create_buffer(gl: &WebGlRenderingContext) -> Result<WebGlBuffer, String> {
    gl.create_buffer().ok_or("failed to create buffer".to_string())
}

pub fn bind_array_buffer(gl: &WebGlRenderingContext, buffer: &WebGlBuffer) {
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
}

pub fn bind_element_array_buffer(gl: &WebGlRenderingContext, buffer: &WebGlBuffer) {
    gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
}

pub fn buffer_data_f32(gl: &WebGlRenderingContext, vertices: &Vec<f32>) {
    unsafe {
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &js_sys::Float32Array::view(vertices),
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
}

pub fn clear_gl(gl: &WebGlRenderingContext) {
    gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
    gl.clear_color(1.0, 1.0, 1.0, 1.0);
    gl.clear_depth(1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
}

pub fn set_attributes(gl: &WebGlRenderingContext, program: &WebGlProgram, attributes: &Vec<Attribute>) {
    attributes.iter().for_each(|attribute| {
        let attribute_location = gl.get_attrib_location(program, attribute.name) as u32;
        gl.enable_vertex_attrib_array(attribute_location);
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&attribute.buffer));
        gl.vertex_attrib_pointer_with_i32(
            attribute_location,
            attribute.num_of_components,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
    })
}

#[rustfmt::skip]
pub fn cube_vertices() -> Vec<f32> {
    vec![
    // Front face
    -1.0, -1.0,  1.0,
    1.0, -1.0,  1.0,
    1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
    // Back face
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
    1.0,  1.0, -1.0,
    1.0, -1.0, -1.0,
    // Top face
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0, -1.0,
    // Bottom face
    -1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    1.0, -1.0,  1.0,
    -1.0, -1.0,  1.0,
    // Right face
    1.0, -1.0, -1.0,
    1.0,  1.0, -1.0,
    1.0,  1.0,  1.0,
    1.0, -1.0,  1.0,
    // Left face
    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0]
}

#[rustfmt::skip]
pub fn cube_indices() -> Vec<u16> {
   vec![
   0,  1,  2,      0,  2,  3,    // front
   4,  5,  6,      4,  6,  7,    // back
   8,  9,  10,     8,  10, 11,   // top
   12, 13, 14,     12, 14, 15,   // bottom
   16, 17, 18,     16, 18, 19,   // right
   20, 21, 22,     20, 22, 23,   // left
  ]
}
