use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::webgl::*;

#[wasm_bindgen]
#[allow(dead_code)]
pub fn mendelbrot() -> Result<(), JsValue> {
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

    clear_gl(&gl);
    gl.use_program(Some(&program));

    let buffer = create_buffer(&gl)?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    buffer_data(&gl, &vec![-1.0, -1.0, 3.0, -1.0, -1.0, 3.0]);

    let attributes = vec![Attribute {
        name: "a_position".into(),
        num_of_components: 2,
        buffer,
    }];
    set_attributes(&gl, &program, &attributes);

    gl.uniform2f(
        Some(&gl.get_uniform_location(&program, "u_zoom_center").unwrap()),
        -0.0,
        -0.0,
    );
    gl.uniform1f(Some(&gl.get_uniform_location(&program, "u_zoom_size").unwrap()), 4.0);
    gl.uniform1i(
        Some(&gl.get_uniform_location(&program, "u_max_iterations").unwrap()),
        500,
    );

    gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3);

    Ok(())
}
