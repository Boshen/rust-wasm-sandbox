#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum AttributeType {
    Scalar,
    Vector(Dimension),
    Matrix(Dimension, Dimension),
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Dimension {
    D2 = 2,
    D3 = 3,
    D4 = 4,
}

#[allow(dead_code)]
pub enum UniformType {
    Scalar(NumberType),
    Vector(NumberType, Dimension),
    Matrix(Dimension),
    Sampler2D,
    Array(Box<UniformType>, usize),
    UserType(String),
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum UniformValue {
    Int(i32),
    Float(f32),
    Vector2([f32; 2]),
    Vector3([f32; 3]),
    Vector4([f32; 4]),
    IVector2([i32; 2]),
    IVector3([i32; 3]),
    IVector4([i32; 4]),
    Matrix2([f32; 4]),
    Matrix3([f32; 9]),
    Matrix4([f32; 16]),
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum NumberType {
    Int,
    Float,
}
