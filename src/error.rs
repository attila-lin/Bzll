#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GlError(pub GLenum);

#[derive(Debug)]
pub enum InitError {
    GlError(GlError),
    CompileFailed(&'static str, String),
    LinkFailed(String),
    ComputeError(compute_shader::error::Error),
    InvalidSetting,
}

#[derive(Debug)]
pub enum RasterError {
    GlError(GlError),
    ComputeError(compute_shader::error::Error),
    UnsupportedImageFormat,
}