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
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    attributes: Vec<Attribute>,

    dots: Vec<Dot>,
    mouse_down: bool,
    mouse_xy: (f32, f32),
    n: i32,
}

impl App {
    pub fn new() -> Result<App, JsValue> {
        let gl = init_gl("canvas")?;
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
            gl,
            program,
            attributes: vec![attribute],
            dots: vec![],
            mouse_down: false,
            mouse_xy: (0.0, 0.0),
            n: 0,
        })
    }

    fn set_translation(&self, dot: &Dot) {
        self.gl.uniform2f(
            Some(&self.gl.get_uniform_location(&self.program, "u_translation").unwrap()),
            dot.x,
            dot.y,
        );
    }

    fn set_scale(&self, dot: &Dot) {
        self.gl.uniform1f(
            Some(&self.gl.get_uniform_location(&self.program, "u_scale").unwrap()),
            dot.r,
        );
    }

    fn set_alpha(&self, dot: &Dot) {
        self.gl.uniform1f(
            Some(&self.gl.get_uniform_location(&self.program, "u_alpha").unwrap()),
            dot.alpha,
        );
    }

    fn set_color(&self, dot: &Dot) {
        self.gl.uniform3fv_with_f32_array(
            Some(&self.gl.get_uniform_location(&self.program, "u_color").unwrap()),
            &dot.color,
        );
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
        clear_gl(&self.gl);
        self.gl.use_program(Some(&self.program));
        set_attributes(&self.gl, &self.program, &self.attributes);
        self.dots.iter().for_each(|dot| {
            self.set_translation(dot);
            self.set_scale(dot);
            self.set_alpha(dot);
            self.set_color(dot);
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

    dom::request_animation_frame(move |_dt| {
        app.borrow_mut().step();
        app.borrow().render();
    });

    Ok(())
}
