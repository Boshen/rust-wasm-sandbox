use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::dom;
use crate::gl::{Attribute, AttributeType, Dimension, Program, ProgramDescription};

struct App {
    program: Program,

    zoom_center: (f32, f32),
    target_zoom_center: (f32, f32),
    zoom_size: f32,
    zoom_factor: f32,
    max_iterations: i32,
    zooming: bool,
}

impl App {
    pub fn new() -> Result<App, JsValue> {
        let vertex_source = r#"
        precision highp float;
        attribute vec2 a_position;
        void main() {
          gl_Position = vec4(a_position, 0.0, 1.0);
        }
    "#;
        let fragment_source = r#"
        precision highp float;

        uniform vec2 u_dimension;
        uniform vec2 u_zoom_center;
        uniform float u_zoom_size;
        uniform int u_max_iterations;

        vec2 f(vec2 x, vec2 c) {
            return mat2(x, -x.y, x.x) * x + c;
        }
        vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
            return a + b * cos(6.2 * (c * t + d));
        }
        void main() {
          vec2 uv = gl_FragCoord.xy / u_dimension;
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
          gl_FragColor = escaped
            ? vec4(palette(float(iterations) / float(u_max_iterations), vec3(0.0),vec3(0.59, 0.55, 0.75), vec3(0.1, 0.2, 0.3), vec3(0.75)), 1.0)
            : vec4(vec3(0.85, 0.99, 1.0), 1.0);
        }
    "#;

        let program = Program::new(
            "canvas",
            ProgramDescription {
                vertex_source,
                fragment_source,
                attributes: vec![Attribute {
                    name: "a_position",
                    attribute_type: AttributeType::Vector(Dimension::D2),
                    vertices: vec![-1.0, -1.0, 3.0, -1.0, -1.0, 3.0],
                    element_array: None,
                }],
            },
        )?;

        Ok(App {
            program,
            zoom_center: (0.0, 0.0),
            target_zoom_center: (0.0, 0.0),
            zoom_size: 4.0,
            zoom_factor: 1.0,
            max_iterations: 500,
            zooming: false,
        })
    }

    pub fn toggle_zooming(&mut self, (x, y): (f32, f32)) {
        self.zooming = !self.zooming;
        if self.zooming {
            self.set_target_zoom_center((x, y));
            self.zoom_factor = 0.96;
            self.max_iterations = 50;
        } else {
            self.max_iterations = 1000;
            self.zoom_factor = 1.0;
        }
    }

    pub fn set_target_zoom_center(&mut self, (x, y): (f32, f32)) {
        self.target_zoom_center.0 = self.zoom_center.0 - self.zoom_size / 2.0 + x * self.zoom_size;
        self.target_zoom_center.1 = self.zoom_center.1 + self.zoom_size / 2.0 - y * self.zoom_size;
    }

    pub fn step(&mut self) {
        if self.zooming {
            self.zoom_size *= self.zoom_factor;
            self.zoom_center.0 += 0.1 * (self.target_zoom_center.0 - self.zoom_center.0);
            self.zoom_center.1 += 0.1 * (self.target_zoom_center.1 - self.zoom_center.1);
        }
    }

    pub fn render(&self) {
        let canvas = dom::canvas("canvas");
        self.program.clear_gl();
        self.program.gl.use_program(Some(&self.program.program));
        self.program.set_attributes();
        self.program.gl.uniform2f(
            self.program
                .gl
                .get_uniform_location(&self.program.program, "u_dimension")
                .as_ref(),
            canvas.width() as f32,
            canvas.height() as f32,
        );
        self.program.gl.uniform2f(
            self.program
                .gl
                .get_uniform_location(&self.program.program, "u_zoom_center")
                .as_ref(),
            self.zoom_center.0,
            self.zoom_center.1,
        );
        self.program.gl.uniform1f(
            self.program
                .gl
                .get_uniform_location(&self.program.program, "u_zoom_size")
                .as_ref(),
            self.zoom_size,
        );
        self.program.gl.uniform1i(
            self.program
                .gl
                .get_uniform_location(&self.program.program, "u_max_iterations")
                .as_ref(),
            self.max_iterations,
        );
        self.program.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn mendelbrot() -> Result<(), JsValue> {
    let app = App::new()?;
    app.render();

    let app = Rc::new(RefCell::new(app));

    let canvas = dom::canvas("canvas");
    let width = canvas.width() as f32;
    let height = canvas.height() as f32;

    {
        let app = app.clone();
        dom::add_mouse_event_listener(&canvas, "click", move |e| {
            let x = e.offset_x() as f32 / width;
            let y = e.offset_y() as f32 / height;
            app.borrow_mut().toggle_zooming((x, y));
        });
    }

    dom::request_animation_frame(move |_t, _dt| {
        app.borrow_mut().step();
        app.borrow().render();
    });

    Ok(())
}
