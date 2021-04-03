use web_sys::WebGlBuffer;

use crate::gl::enums::AttributeType;

pub struct Attribute {
    pub name: &'static str,
    pub attribute_type: AttributeType,
    pub vertices: Vec<f32>,
}

pub struct AttributeLocation {
    pub location: u32,
    pub attribute_type: AttributeType,
    pub buffer: WebGlBuffer,
}

impl AttributeLocation {
    pub fn num_of_components(&self) -> i32 {
        match self.attribute_type {
            AttributeType::Scalar => 1,
            AttributeType::Vector(n) => n as i32,
            AttributeType::Matrix(m, n) => (m as i32) * (n as i32),
        }
    }
}
