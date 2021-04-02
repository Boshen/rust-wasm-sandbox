use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext};

struct App {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    attributes: Vec<Attribute>,
}

use crate::dom;
use crate::webgl;
use crate::webgl::Attribute;

impl App {
    pub fn new() -> Result<App, JsValue> {
        let gl = webgl::init_gl("canvas")?;
        let vertex_source = r#"
        attribute vec2 a_position;
        void main() {
          gl_Position = vec4(position.xy, 0.0, 1.0);
        }
    "#;
        let frag_source = r#"
        precision mediump float;
        void main() {
          gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        }
    "#;
        let program = webgl::create_program(&gl, &vertex_source, &frag_source)?;

        let buffer = webgl::create_buffer(&gl)?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        webgl::buffer_data(&gl, &vec![0.0, 0.0]);

        let attribute = Attribute {
            name: "a_position".into(),
            num_of_components: 2,
            buffer,
        };

        Ok(App {
            gl,
            program,
            attributes: vec![attribute],
        })
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn threed() -> Result<(), JsValue> {
    let app = App::new()?;
    Ok(())
}
