use web_sys::WebGlBuffer;

pub struct Attribute {
    pub name: &'static str,
    pub attribute_type: AttributeType,
    pub vertices: Vec<f32>,
    pub element_array: Option<Vec<u16>>,
}

pub struct AttributeLocation {
    pub location: u32,
    pub attribute_type: AttributeType,
    pub buffer: WebGlBuffer,
    pub element_array_buffer: Option<WebGlBuffer>,
}

#[derive(Copy, Clone)]
pub enum AttributeType {
    Scalar,
    Vector(Dimension),
    Matrix(Dimension, Dimension),
}

#[derive(Copy, Clone)]
pub enum Dimension {
    D2 = 2,
    D3 = 3,
    D4 = 4,
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
