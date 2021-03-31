use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

pub struct Attribute {
    pub name: &'static str,
    pub buffer: WebGlBuffer,
    pub num_of_components: i32,
}

pub fn init_gl() -> Result<WebGlRenderingContext, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    Ok(gl)
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

pub fn create_buffer(gl: &WebGlRenderingContext) -> Result<WebGlBuffer, String> {
    gl.create_buffer().ok_or("failed to create buffer".to_string())
}

pub fn buffer_data(gl: &WebGlRenderingContext, vertices: &Vec<f32>) {
    unsafe {
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &js_sys::Float32Array::view(vertices),
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
}

pub fn clear_gl(gl: &WebGlRenderingContext) {
    gl.clear_color(1.0, 1.0, 1.0, 1.0);
    gl.clear_depth(1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    gl.enable(WebGlRenderingContext::CULL_FACE);
    gl.enable(WebGlRenderingContext::DEPTH_TEST | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    gl.depth_func(WebGlRenderingContext::LEQUAL);
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
