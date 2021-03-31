use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::dom;
use crate::webgl::*;

struct Dot {
    x: f32,
    y: f32,
    r: f32,
    dx: f32,
    dy: f32,
}

struct App {
    dots: Vec<Dot>,
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    attributes: Vec<Attribute>,
    mouse_down: bool,
    mouse_xy: (f32, f32),
}

impl App {
    pub fn new() -> Result<App, JsValue> {
        let gl = init_gl()?;
        let vertex_source = r#"
        attribute vec2 a_position;
        uniform vec2 u_translation;
        uniform float u_scale;
        void main() {
          vec2 position = a_position + u_translation;
          gl_Position = vec4(position.xy, 0.0, 1.0);
          gl_PointSize = u_scale;
        }
    "#;
        let frag_source = r#"
        precision mediump float;
        void main() {
          vec2 cxy = 2.0 * gl_PointCoord - 1.0;
          float r = dot(cxy, cxy);
          if (r > 1.0) {
              discard;
          }
          gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
        let program = create_program(&gl, &vertex_source, &frag_source)?;

        let buffer = create_buffer(&gl)?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        buffer_data(&gl, &vec![0.0, 0.0]);
        let attribute = Attribute {
            name: "a_position".into(),
            num_of_components: 2,
            buffer,
        };

        Ok(App {
            dots: vec![],
            gl,
            program,
            attributes: vec![attribute],
            mouse_down: false,
            mouse_xy: (0.0, 0.0),
        })
    }

    fn set_translation(&self, dot: &Dot) {
        let translation_location = self
            .gl
            .get_uniform_location(&self.program, "u_translation".into())
            .unwrap();
        self.gl.uniform2f(Some(&translation_location), dot.x, dot.y);
    }

    fn set_scale(&self, dot: &Dot) {
        let scale_location = self
            .gl
            .get_uniform_location(&self.program, "u_scale".into())
            .unwrap();
        self.gl.uniform1f(Some(&scale_location), dot.r);
    }

    pub fn set_mouse_xy(&mut self, mouse_xy: (f32, f32)) {
        self.mouse_xy = mouse_xy;
    }

    pub fn set_mousedown(&mut self, is_down: bool) {
        self.mouse_down = is_down;
    }

    pub fn add_dot(&mut self) {
        let (x, y) = self.mouse_xy;
        self.dots.push(Dot {
            x,
            y,
            r: 10.0,
            dx: 0.0,
            dy: 0.0,
        });
    }

    pub fn add_click_dot(&mut self) {
        if !self.mouse_down {
            return;
        }
        let (x, y) = self.mouse_xy;
        let turns = 16 as usize;
        let theta = 360.0 / turns as f32;
        (0..turns).into_iter().for_each(|n| {
            self.dots.push(Dot {
                x,
                y,
                r: 10.0,
                dx: 0.005 * (n as f32 * theta).cos(),
                dy: 0.005 * (n as f32 * theta).sin(),
            });
        })
    }

    fn update_dots(&mut self) {
        self.dots.iter_mut().for_each(|dot| {
            dot.r -= 0.1;
            dot.x += dot.dx;
            dot.y += dot.dy
        })
    }

    fn remove_dots(&mut self) {
        self.dots.retain(|d| d.r > 0.0)
    }

    pub fn step(&mut self) {
        self.add_dot();
        self.add_click_dot();
        self.update_dots();
        self.remove_dots();
    }

    pub fn render(&self) {
        clear_gl(&self.gl);

        self.gl.use_program(Some(&self.program));

        set_attributes(&self.gl, &self.program, &self.attributes);

        self.dots.iter().for_each(|dot| {
            self.set_translation(dot);
            self.set_scale(dot);
            self.gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 1);
        })
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn tracer() -> Result<(), JsValue> {
    let app = App::new()?;
    let app = Rc::new(RefCell::new(app));

    let canvas = dom::canvas("canvas");
    let client_width = canvas.client_width() as f32;
    let client_height = canvas.client_height() as f32;

    {
        let app = app.clone();
        dom::add_mouse_event_listener(&canvas, "mousemove", move |e| {
            let x = e.client_x() as f32 / client_width * 2.0 - 1.0;
            let y = e.client_y() as f32 / client_height * -2.0 + 1.0;
            app.borrow_mut().set_mouse_xy((x, y));
        });
    }

    {
        let app = app.clone();
        dom::add_mouse_event_listener(&canvas, "mousedown", move |_e| {
            app.borrow_mut().set_mousedown(true);
        });
    }

    {
        let app = app.clone();
        dom::add_mouse_event_listener(&canvas, "mouseup", move |_e| {
            app.borrow_mut().set_mousedown(false);
        });
    }

    dom::request_animation_frame(move |_dt| {
        app.borrow_mut().step();
        app.borrow().render();
    });

    Ok(())
}
