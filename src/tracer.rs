use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::dom;
use crate::gl::{Attribute, AttributeType, Dimension, Object, Program, ProgramDescription, UniformValue};

struct Dot {
    x: f32,
    y: f32,
    r: f32,
    dx: f32,
    dy: f32,
    alpha: f32,
    color: [f32; 3],
}

impl Default for Dot {
    fn default() -> Self {
        Dot {
            x: 0.0,
            y: 0.0,
            r: 10.0,
            dx: 0.0,
            dy: 0.0,
            alpha: 1.0,
            color: [1.0, 0.0, 0.0],
        }
    }
}

struct App {
    program: Program,

    dots: Vec<Dot>,
    mouse_down: bool,
    mouse_xy: (f32, f32),
    n: i32,
}

impl App {
    pub fn new() -> Result<App, JsValue> {
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
        let fragment_source = r#"
        precision mediump float;
        uniform float u_alpha;
        uniform vec3 u_color;
        void main() {
          vec2 cxy = 2.0 * gl_PointCoord - 1.0;
          float r = dot(cxy, cxy);
          if (r > 1.0) {
              discard;
          }
          gl_FragColor = vec4(u_color, u_alpha);
        }
    "#;

        let program = Program::new(
            "canvas",
            ProgramDescription {
                vertex_source,
                fragment_source,
                indices: None,
                attributes: vec![Attribute {
                    name: "a_position",
                    attribute_type: AttributeType::Vector(Dimension::D2),
                    vertices: vec![0.0, 0.0],
                }],
                objects: vec![] as Vec<Object>,
            },
        )?;

        Ok(App {
            program,
            dots: vec![],
            mouse_down: false,
            mouse_xy: (0.0, 0.0),
            n: 0,
        })
    }

    fn osc(&self, n: i32) -> i32 {
        if n <= 255 {
            n
        } else {
            255 - (n % 255)
        }
    }

    fn get_color(&self) -> [f32; 3] {
        [
            self.osc(3 * self.n % 510) as f32 / 255.0,
            self.osc(5 * self.n % 510) as f32 / 255.0,
            self.osc(7 * self.n % 510) as f32 / 255.0,
        ]
    }

    pub fn add_dot(&mut self) {
        let (x, y) = self.mouse_xy;
        self.dots.push(Dot {
            x,
            y,
            color: self.get_color(),
            ..Dot::default()
        });
    }

    pub fn add_click_dot(&mut self) {
        if !self.mouse_down {
            return;
        }
        let (x, y) = self.mouse_xy;
        let turns = 12 as usize;
        let theta = std::f32::consts::TAU / turns as f32;
        (0..turns).into_iter().for_each(|n| {
            self.dots.push(Dot {
                x,
                y,
                dx: 0.005 * (n as f32 * theta).cos(),
                dy: 0.005 * (n as f32 * theta).sin(),
                color: self.get_color(),
                ..Dot::default()
            });
        })
    }

    fn update_dots(&mut self) {
        self.dots.iter_mut().for_each(|dot| {
            dot.r -= 0.1;
            dot.x += dot.dx;
            dot.y += dot.dy;
            dot.alpha -= 0.001;
        })
    }

    fn remove_dots(&mut self) {
        self.dots.retain(|d| d.r > 0.0)
    }

    pub fn set_mouse_xy(&mut self, mouse_xy: (f32, f32)) {
        self.mouse_xy = mouse_xy;
    }

    pub fn set_mousedown(&mut self, is_down: bool) {
        self.mouse_down = is_down;
    }

    pub fn step(&mut self) {
        self.n += 1;
        self.add_dot();
        self.add_click_dot();
        self.update_dots();
        self.remove_dots();
    }

    pub fn render(&self) {
        Program::clear_gl(&self.program.gl);
        self.program.gl.use_program(Some(&self.program.program));
        self.program.set_attributes();
        self.dots.iter().for_each(|dot| {
            self.program
                .set_uniform("u_translation", UniformValue::Vector2([dot.x, dot.y]));
            self.program.set_uniform("u_scale", UniformValue::Float(dot.r));
            self.program.set_uniform("u_alpha", UniformValue::Float(dot.alpha));
            self.program.set_uniform("u_color", UniformValue::Vector3(dot.color));
            self.program.gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 1);
        })
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn tracer() -> Result<(), JsValue> {
    let app = App::new()?;
    let app = Rc::new(RefCell::new(app));
    let canvas = dom::canvas("canvas");

    {
        let app = app.clone();
        dom::add_mouse_event_listener(&canvas, "mousemove", move |e| {
            let canvas = dom::canvas("canvas");
            let client_width = canvas.client_width() as f32;
            let client_height = canvas.client_height() as f32;
            let x = e.offset_x() as f32 / client_width * 2.0 - 1.0;
            let y = e.offset_y() as f32 / client_height * -2.0 + 1.0;
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

    dom::request_animation_frame(move |_t, _dt| {
        app.borrow_mut().step();
        app.borrow().render();
    });

    Ok(())
}
