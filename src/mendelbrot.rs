use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::dom;
use crate::webgl::*;

struct App {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    attributes: Vec<Attribute>,

    zoom_center: (f32, f32),
    zoom_size: f32,
    max_iterations: i32,
    zooming: bool,
}

impl App {
    pub fn new() -> Result<App, JsValue> {
        let gl = init_gl()?;
        let vertex_source = r#"
        precision highp float;
        attribute vec2 a_position;
        void main() {
          gl_Position = vec4(a_position.x, a_position.y, 0.0, 1.0);
        }
    "#;
        let frag_source = r#"
        precision highp float;

        uniform vec2 u_zoom_center;
        uniform float u_zoom_size;
        uniform int u_max_iterations;

        vec2 f(vec2 x, vec2 c) {
            return mat2(x, -x.y, x.x) * x + c;
        }
        vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
            return a + b * cos(6.28318 * (c * t + d));
        }
        void main() {
          vec2 uv = gl_FragCoord.xy / vec2(800.0, 800.0);
          vec2 c = u_zoom_center + (uv * 4.0 - vec2(2.0)) * (u_zoom_size / 4.0);
          vec2 x = vec2(0.0);
          bool escaped = false;
          int iterations = 0;
          for (int i = 0; i < 10000; i++) {
            if (i > u_max_iterations) break;
            iterations = i;
            x = f(x, c);
            if (length(x) > 2.0) {
              escaped = true;
              break;
            }
          }
          gl_FragColor = escaped ? vec4(palette(float(iterations)/float(u_max_iterations), vec3(0.0),vec3(0.59,0.55,0.75),vec3(0.1, 0.2, 0.3),vec3(0.75)),1.0) : vec4(vec3(0.85, 0.99, 1.0), 1.0);
        }
    "#;
        let program = create_program(&gl, &vertex_source, &frag_source)?;
        let buffer = create_buffer(&gl)?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        buffer_data(&gl, &vec![-1.0, -1.0, 3.0, -1.0, -1.0, 3.0]);

        let attribute = Attribute {
            name: "a_position".into(),
            num_of_components: 2,
            buffer,
        };

        Ok(App {
            gl,
            program,
            attributes: vec![attribute],
            zoom_center: (0.0, 0.0),
            zoom_size: 4.0,
            max_iterations: 500,
            zooming: false,
        })
    }

    pub fn set_zooming(&mut self, zooming: bool) {
        self.zooming = zooming;
    }

    pub fn set_zoom_center(&mut self, (x, y): (f32, f32)) {
        self.zoom_center.0 = self.zoom_center.0 - self.zoom_size / 2.0 + x * self.zoom_size;
        self.zoom_center.1 = self.zoom_center.1 + self.zoom_size / 2.0 - y * self.zoom_size;
    }

    pub fn step(&mut self) {
        if self.zooming {
            self.zoom_size *= 0.99;
        }
    }

    pub fn render(&self) {
        clear_gl(&self.gl);
        self.gl.use_program(Some(&self.program));
        set_attributes(&self.gl, &self.program, &self.attributes);
        self.gl.uniform2f(
            Some(&self.gl.get_uniform_location(&self.program, "u_zoom_center").unwrap()),
            self.zoom_center.0,
            self.zoom_center.1,
        );
        self.gl.uniform1f(
            Some(&self.gl.get_uniform_location(&self.program, "u_zoom_size").unwrap()),
            self.zoom_size,
        );
        self.gl.uniform1i(
            Some(&self.gl.get_uniform_location(&self.program, "u_max_iterations").unwrap()),
            self.max_iterations,
        );
        self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn mendelbrot() -> Result<(), JsValue> {
    let app = App::new()?;
    app.render();

    let app = Rc::new(RefCell::new(app));

    let canvas = dom::canvas("canvas");
    let client_width = canvas.client_width() as f32;
    let client_height = canvas.client_height() as f32;

    {
        let app = app.clone();
        dom::add_mouse_event_listener(&canvas, "mousedown", move |e| {
            let x = e.offset_x() as f32 / client_width;
            let y = e.offset_y() as f32 / client_height;
            app.borrow_mut().set_zoom_center((x, y));
            app.borrow_mut().set_zooming(true);
        });
    }

    {
        let app = app.clone();
        dom::add_mouse_event_listener(&canvas, "mouseup", move |_e| {
            app.borrow_mut().set_zooming(false);
        });
    }

    dom::request_animation_frame(move |_dt| {
        app.borrow_mut().step();
        app.borrow().render();
    });

    Ok(())
}
