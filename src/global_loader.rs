use super::*;

/// Load each GLES symbol using a provided loader function.
///
/// This allows for the use of functions like `glfwGetProcAddress` or
/// `SDL_GL_GetProcAddress`.
///
/// ```ignore
/// gles30::load_with(|ptr| SDL_GL_GetProcAddress(ptr));
/// ```
///
/// Function loading **will** attempt to use fallback options if the first
/// lookup fails.
///
/// ## Safety
///
/// * This mostly trusts whatever the loader function says, so the loader must
///   provide the correct pointer or a null pointer for each request.
pub unsafe fn load_gles_with<F>(mut load_fn: F)
where
  F: FnMut(*const c_char) -> *const c_void,
{
  load_all_functions(&mut load_fn)
}

// Calls the `load_with` for each function module.
unsafe fn load_all_functions(
  load_fn: &mut dyn FnMut(*const c_char) -> *const c_void,
) {
  ActiveTexture::load_with(&mut *load_fn);
  AttachShader::load_with(&mut *load_fn);
  BeginQuery::load_with(&mut *load_fn);
  BeginTransformFeedback::load_with(&mut *load_fn);
  BindAttribLocation::load_with(&mut *load_fn);
  BindBuffer::load_with(&mut *load_fn);
  BindBufferBase::load_with(&mut *load_fn);
  BindBufferRange::load_with(&mut *load_fn);
  BindFramebuffer::load_with(&mut *load_fn);
  BindRenderbuffer::load_with(&mut *load_fn);
  BindSampler::load_with(&mut *load_fn);
  BindTexture::load_with(&mut *load_fn);
  BindTransformFeedback::load_with(&mut *load_fn);
  BindVertexArray::load_with(&mut *load_fn);
  BlendColor::load_with(&mut *load_fn);
  BlendEquation::load_with(&mut *load_fn);
  BlendEquationSeparate::load_with(&mut *load_fn);
  BlendFunc::load_with(&mut *load_fn);
  BlendFuncSeparate::load_with(&mut *load_fn);
  BlitFramebuffer::load_with(&mut *load_fn);
  BufferData::load_with(&mut *load_fn);
  BufferSubData::load_with(&mut *load_fn);
  CheckFramebufferStatus::load_with(&mut *load_fn);
  Clear::load_with(&mut *load_fn);
  ClearBufferfi::load_with(&mut *load_fn);
  ClearBufferfv::load_with(&mut *load_fn);
  ClearBufferiv::load_with(&mut *load_fn);
  ClearBufferuiv::load_with(&mut *load_fn);
  ClearColor::load_with(&mut *load_fn);
  ClearDepthf::load_with(&mut *load_fn);
  ClearStencil::load_with(&mut *load_fn);
  ClientWaitSync::load_with(&mut *load_fn);
  ColorMask::load_with(&mut *load_fn);
  CompileShader::load_with(&mut *load_fn);
  CompressedTexImage2D::load_with(&mut *load_fn);
  CompressedTexImage3D::load_with(&mut *load_fn);
  CompressedTexSubImage2D::load_with(&mut *load_fn);
  CompressedTexSubImage3D::load_with(&mut *load_fn);
  CopyBufferSubData::load_with(&mut *load_fn);
  CopyTexImage2D::load_with(&mut *load_fn);
  CopyTexSubImage2D::load_with(&mut *load_fn);
  CopyTexSubImage3D::load_with(&mut *load_fn);
  CreateProgram::load_with(&mut *load_fn);
  CreateShader::load_with(&mut *load_fn);
  CullFace::load_with(&mut *load_fn);
  DeleteBuffers::load_with(&mut *load_fn);
  DeleteFramebuffers::load_with(&mut *load_fn);
  DeleteProgram::load_with(&mut *load_fn);
  DeleteQueries::load_with(&mut *load_fn);
  DeleteRenderbuffers::load_with(&mut *load_fn);
  DeleteSamplers::load_with(&mut *load_fn);
  DeleteShader::load_with(&mut *load_fn);
  DeleteSync::load_with(&mut *load_fn);
  DeleteTextures::load_with(&mut *load_fn);
  DeleteTransformFeedbacks::load_with(&mut *load_fn);
  DeleteVertexArrays::load_with(&mut *load_fn);
  DepthFunc::load_with(&mut *load_fn);
  DepthMask::load_with(&mut *load_fn);
  DepthRangef::load_with(&mut *load_fn);
  DetachShader::load_with(&mut *load_fn);
  Disable::load_with(&mut *load_fn);
  DisableVertexAttribArray::load_with(&mut *load_fn);
  DrawArrays::load_with(&mut *load_fn);
  DrawArraysInstanced::load_with(&mut *load_fn);
  DrawBuffers::load_with(&mut *load_fn);
  DrawElements::load_with(&mut *load_fn);
  DrawElementsInstanced::load_with(&mut *load_fn);
  DrawRangeElements::load_with(&mut *load_fn);
  Enable::load_with(&mut *load_fn);
  EnableVertexAttribArray::load_with(&mut *load_fn);
  EndQuery::load_with(&mut *load_fn);
  EndTransformFeedback::load_with(&mut *load_fn);
  FenceSync::load_with(&mut *load_fn);
  Finish::load_with(&mut *load_fn);
  Flush::load_with(&mut *load_fn);
  FlushMappedBufferRange::load_with(&mut *load_fn);
  FramebufferRenderbuffer::load_with(&mut *load_fn);
  FramebufferTexture2D::load_with(&mut *load_fn);
  FramebufferTextureLayer::load_with(&mut *load_fn);
  FrontFace::load_with(&mut *load_fn);
  GenBuffers::load_with(&mut *load_fn);
  GenFramebuffers::load_with(&mut *load_fn);
  GenQueries::load_with(&mut *load_fn);
  GenRenderbuffers::load_with(&mut *load_fn);
  GenSamplers::load_with(&mut *load_fn);
  GenTextures::load_with(&mut *load_fn);
  GenTransformFeedbacks::load_with(&mut *load_fn);
  GenVertexArrays::load_with(&mut *load_fn);
  GenerateMipmap::load_with(&mut *load_fn);
  GetActiveAttrib::load_with(&mut *load_fn);
  GetActiveUniform::load_with(&mut *load_fn);
  GetActiveUniformBlockName::load_with(&mut *load_fn);
  GetActiveUniformBlockiv::load_with(&mut *load_fn);
  GetActiveUniformsiv::load_with(&mut *load_fn);
  GetAttachedShaders::load_with(&mut *load_fn);
  GetAttribLocation::load_with(&mut *load_fn);
  GetBooleanv::load_with(&mut *load_fn);
  GetBufferParameteri64v::load_with(&mut *load_fn);
  GetBufferParameteriv::load_with(&mut *load_fn);
  GetBufferPointerv::load_with(&mut *load_fn);
  GetError::load_with(&mut *load_fn);
  GetFloatv::load_with(&mut *load_fn);
  GetFragDataLocation::load_with(&mut *load_fn);
  GetFramebufferAttachmentParameteriv::load_with(&mut *load_fn);
  GetInteger64i_v::load_with(&mut *load_fn);
  GetInteger64v::load_with(&mut *load_fn);
  GetIntegeri_v::load_with(&mut *load_fn);
  GetIntegerv::load_with(&mut *load_fn);
  GetInternalformativ::load_with(&mut *load_fn);
  GetProgramBinary::load_with(&mut *load_fn);
  GetProgramInfoLog::load_with(&mut *load_fn);
  GetProgramiv::load_with(&mut *load_fn);
  GetQueryObjectuiv::load_with(&mut *load_fn);
  GetQueryiv::load_with(&mut *load_fn);
  GetRenderbufferParameteriv::load_with(&mut *load_fn);
  GetSamplerParameterfv::load_with(&mut *load_fn);
  GetSamplerParameteriv::load_with(&mut *load_fn);
  GetShaderInfoLog::load_with(&mut *load_fn);
  GetShaderPrecisionFormat::load_with(&mut *load_fn);
  GetShaderSource::load_with(&mut *load_fn);
  GetShaderiv::load_with(&mut *load_fn);
  GetString::load_with(&mut *load_fn);
  GetStringi::load_with(&mut *load_fn);
  GetSynciv::load_with(&mut *load_fn);
  GetTexParameterfv::load_with(&mut *load_fn);
  GetTexParameteriv::load_with(&mut *load_fn);
  GetTransformFeedbackVarying::load_with(&mut *load_fn);
  GetUniformBlockIndex::load_with(&mut *load_fn);
  GetUniformIndices::load_with(&mut *load_fn);
  GetUniformLocation::load_with(&mut *load_fn);
  GetUniformfv::load_with(&mut *load_fn);
  GetUniformiv::load_with(&mut *load_fn);
  GetUniformuiv::load_with(&mut *load_fn);
  GetVertexAttribIiv::load_with(&mut *load_fn);
  GetVertexAttribIuiv::load_with(&mut *load_fn);
  GetVertexAttribPointerv::load_with(&mut *load_fn);
  GetVertexAttribfv::load_with(&mut *load_fn);
  GetVertexAttribiv::load_with(&mut *load_fn);
  Hint::load_with(&mut *load_fn);
  InvalidateFramebuffer::load_with(&mut *load_fn);
  InvalidateSubFramebuffer::load_with(&mut *load_fn);
  IsBuffer::load_with(&mut *load_fn);
  IsEnabled::load_with(&mut *load_fn);
  IsFramebuffer::load_with(&mut *load_fn);
  IsProgram::load_with(&mut *load_fn);
  IsQuery::load_with(&mut *load_fn);
  IsRenderbuffer::load_with(&mut *load_fn);
  IsSampler::load_with(&mut *load_fn);
  IsShader::load_with(&mut *load_fn);
  IsSync::load_with(&mut *load_fn);
  IsTexture::load_with(&mut *load_fn);
  IsTransformFeedback::load_with(&mut *load_fn);
  IsVertexArray::load_with(&mut *load_fn);
  LineWidth::load_with(&mut *load_fn);
  LinkProgram::load_with(&mut *load_fn);
  MapBufferRange::load_with(&mut *load_fn);
  PauseTransformFeedback::load_with(&mut *load_fn);
  PixelStorei::load_with(&mut *load_fn);
  PolygonOffset::load_with(&mut *load_fn);
  ProgramBinary::load_with(&mut *load_fn);
  ProgramParameteri::load_with(&mut *load_fn);
  ReadBuffer::load_with(&mut *load_fn);
  ReadPixels::load_with(&mut *load_fn);
  ReleaseShaderCompiler::load_with(&mut *load_fn);
  RenderbufferStorage::load_with(&mut *load_fn);
  RenderbufferStorageMultisample::load_with(&mut *load_fn);
  ResumeTransformFeedback::load_with(&mut *load_fn);
  SampleCoverage::load_with(&mut *load_fn);
  SamplerParameterf::load_with(&mut *load_fn);
  SamplerParameterfv::load_with(&mut *load_fn);
  SamplerParameteri::load_with(&mut *load_fn);
  SamplerParameteriv::load_with(&mut *load_fn);
  Scissor::load_with(&mut *load_fn);
  ShaderBinary::load_with(&mut *load_fn);
  ShaderSource::load_with(&mut *load_fn);
  StencilFunc::load_with(&mut *load_fn);
  StencilFuncSeparate::load_with(&mut *load_fn);
  StencilMask::load_with(&mut *load_fn);
  StencilMaskSeparate::load_with(&mut *load_fn);
  StencilOp::load_with(&mut *load_fn);
  StencilOpSeparate::load_with(&mut *load_fn);
  TexImage2D::load_with(&mut *load_fn);
  TexImage3D::load_with(&mut *load_fn);
  TexParameterf::load_with(&mut *load_fn);
  TexParameterfv::load_with(&mut *load_fn);
  TexParameteri::load_with(&mut *load_fn);
  TexParameteriv::load_with(&mut *load_fn);
  TexStorage2D::load_with(&mut *load_fn);
  TexStorage3D::load_with(&mut *load_fn);
  TexSubImage2D::load_with(&mut *load_fn);
  TexSubImage3D::load_with(&mut *load_fn);
  TransformFeedbackVaryings::load_with(&mut *load_fn);
  Uniform1f::load_with(&mut *load_fn);
  Uniform1fv::load_with(&mut *load_fn);
  Uniform1i::load_with(&mut *load_fn);
  Uniform1iv::load_with(&mut *load_fn);
  Uniform1ui::load_with(&mut *load_fn);
  Uniform1uiv::load_with(&mut *load_fn);
  Uniform2f::load_with(&mut *load_fn);
  Uniform2fv::load_with(&mut *load_fn);
  Uniform2i::load_with(&mut *load_fn);
  Uniform2iv::load_with(&mut *load_fn);
  Uniform2ui::load_with(&mut *load_fn);
  Uniform2uiv::load_with(&mut *load_fn);
  Uniform3f::load_with(&mut *load_fn);
  Uniform3fv::load_with(&mut *load_fn);
  Uniform3i::load_with(&mut *load_fn);
  Uniform3iv::load_with(&mut *load_fn);
  Uniform3ui::load_with(&mut *load_fn);
  Uniform3uiv::load_with(&mut *load_fn);
  Uniform4f::load_with(&mut *load_fn);
  Uniform4fv::load_with(&mut *load_fn);
  Uniform4i::load_with(&mut *load_fn);
  Uniform4iv::load_with(&mut *load_fn);
  Uniform4ui::load_with(&mut *load_fn);
  Uniform4uiv::load_with(&mut *load_fn);
  UniformBlockBinding::load_with(&mut *load_fn);
  UniformMatrix2fv::load_with(&mut *load_fn);
  UniformMatrix2x3fv::load_with(&mut *load_fn);
  UniformMatrix2x4fv::load_with(&mut *load_fn);
  UniformMatrix3fv::load_with(&mut *load_fn);
  UniformMatrix3x2fv::load_with(&mut *load_fn);
  UniformMatrix3x4fv::load_with(&mut *load_fn);
  UniformMatrix4fv::load_with(&mut *load_fn);
  UniformMatrix4x2fv::load_with(&mut *load_fn);
  UniformMatrix4x3fv::load_with(&mut *load_fn);
  UnmapBuffer::load_with(&mut *load_fn);
  UseProgram::load_with(&mut *load_fn);
  ValidateProgram::load_with(&mut *load_fn);
  VertexAttrib1f::load_with(&mut *load_fn);
  VertexAttrib1fv::load_with(&mut *load_fn);
  VertexAttrib2f::load_with(&mut *load_fn);
  VertexAttrib2fv::load_with(&mut *load_fn);
  VertexAttrib3f::load_with(&mut *load_fn);
  VertexAttrib3fv::load_with(&mut *load_fn);
  VertexAttrib4f::load_with(&mut *load_fn);
  VertexAttrib4fv::load_with(&mut *load_fn);
  VertexAttribDivisor::load_with(&mut *load_fn);
  VertexAttribI4i::load_with(&mut *load_fn);
  VertexAttribI4iv::load_with(&mut *load_fn);
  VertexAttribI4ui::load_with(&mut *load_fn);
  VertexAttribI4uiv::load_with(&mut *load_fn);
  VertexAttribIPointer::load_with(&mut *load_fn);
  VertexAttribPointer::load_with(&mut *load_fn);
  Viewport::load_with(&mut *load_fn);
  WaitSync::load_with(&mut *load_fn);
}

/// Function pointer sanity check.
///
/// * Null pointers (0) are bad.
/// * Sometimes windows will return non-null error values.
///   * Known non-null error values include 1, 2, 3, and -1.
fn fn_ptr_ok(p: *const c_void) -> bool {
  let p_u = p as usize;
  (p_u >= 8) && (p_u != (-1_isize) as usize)
}

unsafe fn meta_loader(
  loader: &mut dyn FnMut(*const c_char) -> *const c_void,
  names: &[&[u8]],
) -> OptVoidPtr {
  for name in names.iter() {
    debug_assert!(*name.iter().last().unwrap() == 0_u8);
    let p = loader(name.as_ptr() as *const c_char);
    if fn_ptr_ok(p) {
      return NonNull::new(p as *mut c_void);
    }
  }
  None
}

pub use types::*;
pub mod types {
  use super::*;
  // Common types from OpenGL 1.1
  pub type GLenum = c_uint;
  pub type GLboolean = c_uchar;
  pub type GLbitfield = c_uint;
  pub type GLvoid = c_void;
  pub type GLbyte = c_char;
  pub type GLshort = c_short;
  pub type GLint = c_int;
  pub type GLclampx = c_int;
  pub type GLubyte = c_uchar;
  pub type GLushort = c_ushort;
  pub type GLuint = c_uint;
  pub type GLsizei = c_int;
  pub type GLfloat = c_float;
  pub type GLclampf = c_float;
  pub type GLdouble = c_double;
  pub type GLclampd = c_double;
  pub type GLeglImageOES = *const c_void;
  pub type GLchar = c_char;
  pub type GLcharARB = c_char;

  #[cfg(target_os = "macos")]
  pub type GLhandleARB = *const c_void;
  #[cfg(not(target_os = "macos"))]
  pub type GLhandleARB = c_uint;

  pub type GLhalfARB = c_ushort;
  pub type GLhalf = c_ushort;

  // Must be 32 bits
  pub type GLfixed = GLint;

  pub type GLintptr = isize;
  pub type GLsizeiptr = isize;
  pub type GLint64 = i64;
  pub type GLuint64 = u64;
  pub type GLintptrARB = isize;
  pub type GLsizeiptrARB = isize;
  pub type GLint64EXT = i64;
  pub type GLuint64EXT = u64;

  pub enum __GLsync {}
  pub type GLsync = *const __GLsync;

  // compatible with OpenCL cl_context
  pub enum _cl_context {}
  pub enum _cl_event {}

  pub type GLDEBUGPROC = Option<
    extern "system" fn(
      source: GLenum,
      gltype: GLenum,
      id: GLuint,
      severity: GLenum,
      length: GLsizei,
      message: *const GLchar,
      userParam: *mut c_void,
    ),
  >;
  pub type GLDEBUGPROCARB = Option<
    extern "system" fn(
      source: GLenum,
      gltype: GLenum,
      id: GLuint,
      severity: GLenum,
      length: GLsizei,
      message: *const GLchar,
      userParam: *mut c_void,
    ),
  >;
  pub type GLDEBUGPROCKHR = Option<
    extern "system" fn(
      source: GLenum,
      gltype: GLenum,
      id: GLuint,
      severity: GLenum,
      length: GLsizei,
      message: *const GLchar,
      userParam: *mut c_void,
    ),
  >;

  // Vendor extension types
  pub type GLDEBUGPROCAMD = Option<
    extern "system" fn(
      id: GLuint,
      category: GLenum,
      severity: GLenum,
      length: GLsizei,
      message: *const GLchar,
      userParam: *mut c_void,
    ),
  >;
  pub type GLhalfNV = c_ushort;
  pub type GLvdpauSurfaceNV = GLintptr;
}
pub use consts::*;
pub mod consts {
  use super::*;
  pub const GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
  pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
  pub const GL_ACTIVE_TEXTURE: GLenum = 0x84E0;
  pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;
  pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
  pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
  pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
  pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
  pub const GL_ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
  pub const GL_ALPHA: GLenum = 0x1906;
  pub const GL_ALPHA_BITS: GLenum = 0x0D55;
  pub const GL_ALREADY_SIGNALED: GLenum = 0x911A;
  pub const GL_ALWAYS: GLenum = 0x0207;
  pub const GL_ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
  pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
  pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
  pub const GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894;
  pub const GL_ATTACHED_SHADERS: GLenum = 0x8B85;
  pub const GL_BACK: GLenum = 0x0405;
  pub const GL_BLEND: GLenum = 0x0BE2;
  pub const GL_BLEND_COLOR: GLenum = 0x8005;
  pub const GL_BLEND_DST_ALPHA: GLenum = 0x80CA;
  pub const GL_BLEND_DST_RGB: GLenum = 0x80C8;
  pub const GL_BLEND_EQUATION: GLenum = 0x8009;
  pub const GL_BLEND_EQUATION_ALPHA: GLenum = 0x883D;
  pub const GL_BLEND_EQUATION_RGB: GLenum = 0x8009;
  pub const GL_BLEND_SRC_ALPHA: GLenum = 0x80CB;
  pub const GL_BLEND_SRC_RGB: GLenum = 0x80C9;
  pub const GL_BLUE: GLenum = 0x1905;
  pub const GL_BLUE_BITS: GLenum = 0x0D54;
  pub const GL_BOOL: GLenum = 0x8B56;
  pub const GL_BOOL_VEC2: GLenum = 0x8B57;
  pub const GL_BOOL_VEC3: GLenum = 0x8B58;
  pub const GL_BOOL_VEC4: GLenum = 0x8B59;
  pub const GL_BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
  pub const GL_BUFFER_MAPPED: GLenum = 0x88BC;
  pub const GL_BUFFER_MAP_LENGTH: GLenum = 0x9120;
  pub const GL_BUFFER_MAP_OFFSET: GLenum = 0x9121;
  pub const GL_BUFFER_MAP_POINTER: GLenum = 0x88BD;
  pub const GL_BUFFER_SIZE: GLenum = 0x8764;
  pub const GL_BUFFER_USAGE: GLenum = 0x8765;
  pub const GL_BYTE: GLenum = 0x1400;
  pub const GL_CCW: GLenum = 0x0901;
  pub const GL_CLAMP_TO_EDGE: GLenum = 0x812F;
  pub const GL_COLOR: GLenum = 0x1800;
  pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
  pub const GL_COLOR_ATTACHMENT1: GLenum = 0x8CE1;
  pub const GL_COLOR_ATTACHMENT10: GLenum = 0x8CEA;
  pub const GL_COLOR_ATTACHMENT11: GLenum = 0x8CEB;
  pub const GL_COLOR_ATTACHMENT12: GLenum = 0x8CEC;
  pub const GL_COLOR_ATTACHMENT13: GLenum = 0x8CED;
  pub const GL_COLOR_ATTACHMENT14: GLenum = 0x8CEE;
  pub const GL_COLOR_ATTACHMENT15: GLenum = 0x8CEF;
  pub const GL_COLOR_ATTACHMENT16: GLenum = 0x8CF0;
  pub const GL_COLOR_ATTACHMENT17: GLenum = 0x8CF1;
  pub const GL_COLOR_ATTACHMENT18: GLenum = 0x8CF2;
  pub const GL_COLOR_ATTACHMENT19: GLenum = 0x8CF3;
  pub const GL_COLOR_ATTACHMENT2: GLenum = 0x8CE2;
  pub const GL_COLOR_ATTACHMENT20: GLenum = 0x8CF4;
  pub const GL_COLOR_ATTACHMENT21: GLenum = 0x8CF5;
  pub const GL_COLOR_ATTACHMENT22: GLenum = 0x8CF6;
  pub const GL_COLOR_ATTACHMENT23: GLenum = 0x8CF7;
  pub const GL_COLOR_ATTACHMENT24: GLenum = 0x8CF8;
  pub const GL_COLOR_ATTACHMENT25: GLenum = 0x8CF9;
  pub const GL_COLOR_ATTACHMENT26: GLenum = 0x8CFA;
  pub const GL_COLOR_ATTACHMENT27: GLenum = 0x8CFB;
  pub const GL_COLOR_ATTACHMENT28: GLenum = 0x8CFC;
  pub const GL_COLOR_ATTACHMENT29: GLenum = 0x8CFD;
  pub const GL_COLOR_ATTACHMENT3: GLenum = 0x8CE3;
  pub const GL_COLOR_ATTACHMENT30: GLenum = 0x8CFE;
  pub const GL_COLOR_ATTACHMENT31: GLenum = 0x8CFF;
  pub const GL_COLOR_ATTACHMENT4: GLenum = 0x8CE4;
  pub const GL_COLOR_ATTACHMENT5: GLenum = 0x8CE5;
  pub const GL_COLOR_ATTACHMENT6: GLenum = 0x8CE6;
  pub const GL_COLOR_ATTACHMENT7: GLenum = 0x8CE7;
  pub const GL_COLOR_ATTACHMENT8: GLenum = 0x8CE8;
  pub const GL_COLOR_ATTACHMENT9: GLenum = 0x8CE9;
  pub const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;
  pub const GL_COLOR_CLEAR_VALUE: GLenum = 0x0C22;
  pub const GL_COLOR_WRITEMASK: GLenum = 0x0C23;
  pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
  pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
  pub const GL_COMPRESSED_R11_EAC: GLenum = 0x9270;
  pub const GL_COMPRESSED_RG11_EAC: GLenum = 0x9272;
  pub const GL_COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
  pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
  pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
  pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
  pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
  pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
  pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
  pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
  pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
  pub const GL_CONDITION_SATISFIED: GLenum = 0x911C;
  pub const GL_CONSTANT_ALPHA: GLenum = 0x8003;
  pub const GL_CONSTANT_COLOR: GLenum = 0x8001;
  pub const GL_COPY_READ_BUFFER: GLenum = 0x8F36;
  pub const GL_COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
  pub const GL_COPY_WRITE_BUFFER: GLenum = 0x8F37;
  pub const GL_COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
  pub const GL_CULL_FACE: GLenum = 0x0B44;
  pub const GL_CULL_FACE_MODE: GLenum = 0x0B45;
  pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;
  pub const GL_CURRENT_QUERY: GLenum = 0x8865;
  pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
  pub const GL_CW: GLenum = 0x0900;
  pub const GL_DECR: GLenum = 0x1E03;
  pub const GL_DECR_WRAP: GLenum = 0x8508;
  pub const GL_DELETE_STATUS: GLenum = 0x8B80;
  pub const GL_DEPTH: GLenum = 0x1801;
  pub const GL_DEPTH24_STENCIL8: GLenum = 0x88F0;
  pub const GL_DEPTH32F_STENCIL8: GLenum = 0x8CAD;
  pub const GL_DEPTH_ATTACHMENT: GLenum = 0x8D00;
  pub const GL_DEPTH_BITS: GLenum = 0x0D56;
  pub const GL_DEPTH_BUFFER_BIT: GLenum = 0x00000100;
  pub const GL_DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
  pub const GL_DEPTH_COMPONENT: GLenum = 0x1902;
  pub const GL_DEPTH_COMPONENT16: GLenum = 0x81A5;
  pub const GL_DEPTH_COMPONENT24: GLenum = 0x81A6;
  pub const GL_DEPTH_COMPONENT32F: GLenum = 0x8CAC;
  pub const GL_DEPTH_FUNC: GLenum = 0x0B74;
  pub const GL_DEPTH_RANGE: GLenum = 0x0B70;
  pub const GL_DEPTH_STENCIL: GLenum = 0x84F9;
  pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
  pub const GL_DEPTH_TEST: GLenum = 0x0B71;
  pub const GL_DEPTH_WRITEMASK: GLenum = 0x0B72;
  pub const GL_DITHER: GLenum = 0x0BD0;
  pub const GL_DONT_CARE: GLenum = 0x1100;
  pub const GL_DRAW_BUFFER0: GLenum = 0x8825;
  pub const GL_DRAW_BUFFER1: GLenum = 0x8826;
  pub const GL_DRAW_BUFFER10: GLenum = 0x882F;
  pub const GL_DRAW_BUFFER11: GLenum = 0x8830;
  pub const GL_DRAW_BUFFER12: GLenum = 0x8831;
  pub const GL_DRAW_BUFFER13: GLenum = 0x8832;
  pub const GL_DRAW_BUFFER14: GLenum = 0x8833;
  pub const GL_DRAW_BUFFER15: GLenum = 0x8834;
  pub const GL_DRAW_BUFFER2: GLenum = 0x8827;
  pub const GL_DRAW_BUFFER3: GLenum = 0x8828;
  pub const GL_DRAW_BUFFER4: GLenum = 0x8829;
  pub const GL_DRAW_BUFFER5: GLenum = 0x882A;
  pub const GL_DRAW_BUFFER6: GLenum = 0x882B;
  pub const GL_DRAW_BUFFER7: GLenum = 0x882C;
  pub const GL_DRAW_BUFFER8: GLenum = 0x882D;
  pub const GL_DRAW_BUFFER9: GLenum = 0x882E;
  pub const GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
  pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
  pub const GL_DST_ALPHA: GLenum = 0x0304;
  pub const GL_DST_COLOR: GLenum = 0x0306;
  pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;
  pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
  pub const GL_DYNAMIC_READ: GLenum = 0x88E9;
  pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
  pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
  pub const GL_EQUAL: GLenum = 0x0202;
  pub const GL_EXTENSIONS: GLenum = 0x1F03;
  pub const GL_FALSE: GLboolean = 0;
  pub const GL_FASTEST: GLenum = 0x1101;
  pub const GL_FIXED: GLenum = 0x140C;
  pub const GL_FLOAT: GLenum = 0x1406;
  pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
  pub const GL_FLOAT_MAT2: GLenum = 0x8B5A;
  pub const GL_FLOAT_MAT2x3: GLenum = 0x8B65;
  pub const GL_FLOAT_MAT2x4: GLenum = 0x8B66;
  pub const GL_FLOAT_MAT3: GLenum = 0x8B5B;
  pub const GL_FLOAT_MAT3x2: GLenum = 0x8B67;
  pub const GL_FLOAT_MAT3x4: GLenum = 0x8B68;
  pub const GL_FLOAT_MAT4: GLenum = 0x8B5C;
  pub const GL_FLOAT_MAT4x2: GLenum = 0x8B69;
  pub const GL_FLOAT_MAT4x3: GLenum = 0x8B6A;
  pub const GL_FLOAT_VEC2: GLenum = 0x8B50;
  pub const GL_FLOAT_VEC3: GLenum = 0x8B51;
  pub const GL_FLOAT_VEC4: GLenum = 0x8B52;
  pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
  pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
  pub const GL_FRAMEBUFFER: GLenum = 0x8D40;
  pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
  pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
  pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
  pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
  pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
  pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
  pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
  pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
  pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
  pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
  pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
  pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
  pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
  pub const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
  pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
  pub const GL_FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
  pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
  pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 0x8CD9;
  pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
  pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
  pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
  pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
  pub const GL_FRONT: GLenum = 0x0404;
  pub const GL_FRONT_AND_BACK: GLenum = 0x0408;
  pub const GL_FRONT_FACE: GLenum = 0x0B46;
  pub const GL_FUNC_ADD: GLenum = 0x8006;
  pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
  pub const GL_FUNC_SUBTRACT: GLenum = 0x800A;
  pub const GL_GENERATE_MIPMAP_HINT: GLenum = 0x8192;
  pub const GL_GEQUAL: GLenum = 0x0206;
  pub const GL_GREATER: GLenum = 0x0204;
  pub const GL_GREEN: GLenum = 0x1904;
  pub const GL_GREEN_BITS: GLenum = 0x0D53;
  pub const GL_HALF_FLOAT: GLenum = 0x140B;
  pub const GL_HIGH_FLOAT: GLenum = 0x8DF2;
  pub const GL_HIGH_INT: GLenum = 0x8DF5;
  pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
  pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
  pub const GL_INCR: GLenum = 0x1E02;
  pub const GL_INCR_WRAP: GLenum = 0x8507;
  pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
  pub const GL_INT: GLenum = 0x1404;
  pub const GL_INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
  pub const GL_INT_2_10_10_10_REV: GLenum = 0x8D9F;
  pub const GL_INT_SAMPLER_2D: GLenum = 0x8DCA;
  pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
  pub const GL_INT_SAMPLER_3D: GLenum = 0x8DCB;
  pub const GL_INT_SAMPLER_CUBE: GLenum = 0x8DCC;
  pub const GL_INT_VEC2: GLenum = 0x8B53;
  pub const GL_INT_VEC3: GLenum = 0x8B54;
  pub const GL_INT_VEC4: GLenum = 0x8B55;
  pub const GL_INVALID_ENUM: GLenum = 0x0500;
  pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
  pub const GL_INVALID_INDEX: GLuint = 0xFFFFFFFF;
  pub const GL_INVALID_OPERATION: GLenum = 0x0502;
  pub const GL_INVALID_VALUE: GLenum = 0x0501;
  pub const GL_INVERT: GLenum = 0x150A;
  pub const GL_KEEP: GLenum = 0x1E00;
  pub const GL_LEQUAL: GLenum = 0x0203;
  pub const GL_LESS: GLenum = 0x0201;
  pub const GL_LINEAR: GLenum = 0x2601;
  pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
  pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
  pub const GL_LINES: GLenum = 0x0001;
  pub const GL_LINE_LOOP: GLenum = 0x0002;
  pub const GL_LINE_STRIP: GLenum = 0x0003;
  pub const GL_LINE_WIDTH: GLenum = 0x0B21;
  pub const GL_LINK_STATUS: GLenum = 0x8B82;
  pub const GL_LOW_FLOAT: GLenum = 0x8DF0;
  pub const GL_LOW_INT: GLenum = 0x8DF3;
  pub const GL_LUMINANCE: GLenum = 0x1909;
  pub const GL_LUMINANCE_ALPHA: GLenum = 0x190A;
  pub const GL_MAJOR_VERSION: GLenum = 0x821B;
  pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
  pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
  pub const GL_MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
  pub const GL_MAP_READ_BIT: GLenum = 0x0001;
  pub const GL_MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
  pub const GL_MAP_WRITE_BIT: GLenum = 0x0002;
  pub const GL_MAX: GLenum = 0x8008;
  pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
  pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
  pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
  pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
  pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
  pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
  pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
  pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
  pub const GL_MAX_DRAW_BUFFERS: GLenum = 0x8824;
  pub const GL_MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
  pub const GL_MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
  pub const GL_MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
  pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
  pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
  pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
  pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
  pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
  pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
  pub const GL_MAX_SAMPLES: GLenum = 0x8D57;
  pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
  pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
  pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
  pub const GL_MAX_TEXTURE_SIZE: GLenum = 0x0D33;
  pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
  pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
  pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
  pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
  pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
  pub const GL_MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
  pub const GL_MAX_VARYING_VECTORS: GLenum = 0x8DFC;
  pub const GL_MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
  pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
  pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
  pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
  pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
  pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
  pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
  pub const GL_MEDIUM_FLOAT: GLenum = 0x8DF1;
  pub const GL_MEDIUM_INT: GLenum = 0x8DF4;
  pub const GL_MIN: GLenum = 0x8007;
  pub const GL_MINOR_VERSION: GLenum = 0x821C;
  pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
  pub const GL_MIRRORED_REPEAT: GLenum = 0x8370;
  pub const GL_NEAREST: GLenum = 0x2600;
  pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
  pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
  pub const GL_NEVER: GLenum = 0x0200;
  pub const GL_NICEST: GLenum = 0x1102;
  pub const GL_NONE: GLenum = 0;
  pub const GL_NOTEQUAL: GLenum = 0x0205;
  pub const GL_NO_ERROR: GLenum = 0;
  pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
  pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;
  pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
  pub const GL_NUM_SAMPLE_COUNTS: GLenum = 0x9380;
  pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
  pub const GL_OBJECT_TYPE: GLenum = 0x9112;
  pub const GL_ONE: GLenum = 1;
  pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
  pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
  pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
  pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
  pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
  pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
  pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;
  pub const GL_PACK_ALIGNMENT: GLenum = 0x0D05;
  pub const GL_PACK_ROW_LENGTH: GLenum = 0x0D02;
  pub const GL_PACK_SKIP_PIXELS: GLenum = 0x0D04;
  pub const GL_PACK_SKIP_ROWS: GLenum = 0x0D03;
  pub const GL_PIXEL_PACK_BUFFER: GLenum = 0x88EB;
  pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
  pub const GL_PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
  pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
  pub const GL_POINTS: GLenum = 0x0000;
  pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
  pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;
  pub const GL_POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
  pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
  pub const GL_PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
  pub const GL_PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
  pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
  pub const GL_QUERY_RESULT: GLenum = 0x8866;
  pub const GL_QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
  pub const GL_R11F_G11F_B10F: GLenum = 0x8C3A;
  pub const GL_R16F: GLenum = 0x822D;
  pub const GL_R16I: GLenum = 0x8233;
  pub const GL_R16UI: GLenum = 0x8234;
  pub const GL_R32F: GLenum = 0x822E;
  pub const GL_R32I: GLenum = 0x8235;
  pub const GL_R32UI: GLenum = 0x8236;
  pub const GL_R8: GLenum = 0x8229;
  pub const GL_R8I: GLenum = 0x8231;
  pub const GL_R8UI: GLenum = 0x8232;
  pub const GL_R8_SNORM: GLenum = 0x8F94;
  pub const GL_RASTERIZER_DISCARD: GLenum = 0x8C89;
  pub const GL_READ_BUFFER: GLenum = 0x0C02;
  pub const GL_READ_FRAMEBUFFER: GLenum = 0x8CA8;
  pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
  pub const GL_RED: GLenum = 0x1903;
  pub const GL_RED_BITS: GLenum = 0x0D52;
  pub const GL_RED_INTEGER: GLenum = 0x8D94;
  pub const GL_RENDERBUFFER: GLenum = 0x8D41;
  pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
  pub const GL_RENDERBUFFER_BINDING: GLenum = 0x8CA7;
  pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
  pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
  pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
  pub const GL_RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
  pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
  pub const GL_RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
  pub const GL_RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
  pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
  pub const GL_RENDERBUFFER_WIDTH: GLenum = 0x8D42;
  pub const GL_RENDERER: GLenum = 0x1F01;
  pub const GL_REPEAT: GLenum = 0x2901;
  pub const GL_REPLACE: GLenum = 0x1E01;
  pub const GL_RG: GLenum = 0x8227;
  pub const GL_RG16F: GLenum = 0x822F;
  pub const GL_RG16I: GLenum = 0x8239;
  pub const GL_RG16UI: GLenum = 0x823A;
  pub const GL_RG32F: GLenum = 0x8230;
  pub const GL_RG32I: GLenum = 0x823B;
  pub const GL_RG32UI: GLenum = 0x823C;
  pub const GL_RG8: GLenum = 0x822B;
  pub const GL_RG8I: GLenum = 0x8237;
  pub const GL_RG8UI: GLenum = 0x8238;
  pub const GL_RG8_SNORM: GLenum = 0x8F95;
  pub const GL_RGB: GLenum = 0x1907;
  pub const GL_RGB10_A2: GLenum = 0x8059;
  pub const GL_RGB10_A2UI: GLenum = 0x906F;
  pub const GL_RGB16F: GLenum = 0x881B;
  pub const GL_RGB16I: GLenum = 0x8D89;
  pub const GL_RGB16UI: GLenum = 0x8D77;
  pub const GL_RGB32F: GLenum = 0x8815;
  pub const GL_RGB32I: GLenum = 0x8D83;
  pub const GL_RGB32UI: GLenum = 0x8D71;
  pub const GL_RGB565: GLenum = 0x8D62;
  pub const GL_RGB5_A1: GLenum = 0x8057;
  pub const GL_RGB8: GLenum = 0x8051;
  pub const GL_RGB8I: GLenum = 0x8D8F;
  pub const GL_RGB8UI: GLenum = 0x8D7D;
  pub const GL_RGB8_SNORM: GLenum = 0x8F96;
  pub const GL_RGB9_E5: GLenum = 0x8C3D;
  pub const GL_RGBA: GLenum = 0x1908;
  pub const GL_RGBA16F: GLenum = 0x881A;
  pub const GL_RGBA16I: GLenum = 0x8D88;
  pub const GL_RGBA16UI: GLenum = 0x8D76;
  pub const GL_RGBA32F: GLenum = 0x8814;
  pub const GL_RGBA32I: GLenum = 0x8D82;
  pub const GL_RGBA32UI: GLenum = 0x8D70;
  pub const GL_RGBA4: GLenum = 0x8056;
  pub const GL_RGBA8: GLenum = 0x8058;
  pub const GL_RGBA8I: GLenum = 0x8D8E;
  pub const GL_RGBA8UI: GLenum = 0x8D7C;
  pub const GL_RGBA8_SNORM: GLenum = 0x8F97;
  pub const GL_RGBA_INTEGER: GLenum = 0x8D99;
  pub const GL_RGB_INTEGER: GLenum = 0x8D98;
  pub const GL_RG_INTEGER: GLenum = 0x8228;
  pub const GL_SAMPLER_2D: GLenum = 0x8B5E;
  pub const GL_SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
  pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
  pub const GL_SAMPLER_2D_SHADOW: GLenum = 0x8B62;
  pub const GL_SAMPLER_3D: GLenum = 0x8B5F;
  pub const GL_SAMPLER_BINDING: GLenum = 0x8919;
  pub const GL_SAMPLER_CUBE: GLenum = 0x8B60;
  pub const GL_SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
  pub const GL_SAMPLES: GLenum = 0x80A9;
  pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
  pub const GL_SAMPLE_BUFFERS: GLenum = 0x80A8;
  pub const GL_SAMPLE_COVERAGE: GLenum = 0x80A0;
  pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
  pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
  pub const GL_SCISSOR_BOX: GLenum = 0x0C10;
  pub const GL_SCISSOR_TEST: GLenum = 0x0C11;
  pub const GL_SEPARATE_ATTRIBS: GLenum = 0x8C8D;
  pub const GL_SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
  pub const GL_SHADER_COMPILER: GLenum = 0x8DFA;
  pub const GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
  pub const GL_SHADER_TYPE: GLenum = 0x8B4F;
  pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
  pub const GL_SHORT: GLenum = 0x1402;
  pub const GL_SIGNALED: GLenum = 0x9119;
  pub const GL_SIGNED_NORMALIZED: GLenum = 0x8F9C;
  pub const GL_SRC_ALPHA: GLenum = 0x0302;
  pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;
  pub const GL_SRC_COLOR: GLenum = 0x0300;
  pub const GL_SRGB: GLenum = 0x8C40;
  pub const GL_SRGB8: GLenum = 0x8C41;
  pub const GL_SRGB8_ALPHA8: GLenum = 0x8C43;
  pub const GL_STATIC_COPY: GLenum = 0x88E6;
  pub const GL_STATIC_DRAW: GLenum = 0x88E4;
  pub const GL_STATIC_READ: GLenum = 0x88E5;
  pub const GL_STENCIL: GLenum = 0x1802;
  pub const GL_STENCIL_ATTACHMENT: GLenum = 0x8D20;
  pub const GL_STENCIL_BACK_FAIL: GLenum = 0x8801;
  pub const GL_STENCIL_BACK_FUNC: GLenum = 0x8800;
  pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
  pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
  pub const GL_STENCIL_BACK_REF: GLenum = 0x8CA3;
  pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
  pub const GL_STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
  pub const GL_STENCIL_BITS: GLenum = 0x0D57;
  pub const GL_STENCIL_BUFFER_BIT: GLenum = 0x00000400;
  pub const GL_STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
  pub const GL_STENCIL_FAIL: GLenum = 0x0B94;
  pub const GL_STENCIL_FUNC: GLenum = 0x0B92;
  pub const GL_STENCIL_INDEX8: GLenum = 0x8D48;
  pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
  pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
  pub const GL_STENCIL_REF: GLenum = 0x0B97;
  pub const GL_STENCIL_TEST: GLenum = 0x0B90;
  pub const GL_STENCIL_VALUE_MASK: GLenum = 0x0B93;
  pub const GL_STENCIL_WRITEMASK: GLenum = 0x0B98;
  pub const GL_STREAM_COPY: GLenum = 0x88E2;
  pub const GL_STREAM_DRAW: GLenum = 0x88E0;
  pub const GL_STREAM_READ: GLenum = 0x88E1;
  pub const GL_SUBPIXEL_BITS: GLenum = 0x0D50;
  pub const GL_SYNC_CONDITION: GLenum = 0x9113;
  pub const GL_SYNC_FENCE: GLenum = 0x9116;
  pub const GL_SYNC_FLAGS: GLenum = 0x9115;
  pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
  pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
  pub const GL_SYNC_STATUS: GLenum = 0x9114;
  pub const GL_TEXTURE: GLenum = 0x1702;
  pub const GL_TEXTURE0: GLenum = 0x84C0;
  pub const GL_TEXTURE1: GLenum = 0x84C1;
  pub const GL_TEXTURE10: GLenum = 0x84CA;
  pub const GL_TEXTURE11: GLenum = 0x84CB;
  pub const GL_TEXTURE12: GLenum = 0x84CC;
  pub const GL_TEXTURE13: GLenum = 0x84CD;
  pub const GL_TEXTURE14: GLenum = 0x84CE;
  pub const GL_TEXTURE15: GLenum = 0x84CF;
  pub const GL_TEXTURE16: GLenum = 0x84D0;
  pub const GL_TEXTURE17: GLenum = 0x84D1;
  pub const GL_TEXTURE18: GLenum = 0x84D2;
  pub const GL_TEXTURE19: GLenum = 0x84D3;
  pub const GL_TEXTURE2: GLenum = 0x84C2;
  pub const GL_TEXTURE20: GLenum = 0x84D4;
  pub const GL_TEXTURE21: GLenum = 0x84D5;
  pub const GL_TEXTURE22: GLenum = 0x84D6;
  pub const GL_TEXTURE23: GLenum = 0x84D7;
  pub const GL_TEXTURE24: GLenum = 0x84D8;
  pub const GL_TEXTURE25: GLenum = 0x84D9;
  pub const GL_TEXTURE26: GLenum = 0x84DA;
  pub const GL_TEXTURE27: GLenum = 0x84DB;
  pub const GL_TEXTURE28: GLenum = 0x84DC;
  pub const GL_TEXTURE29: GLenum = 0x84DD;
  pub const GL_TEXTURE3: GLenum = 0x84C3;
  pub const GL_TEXTURE30: GLenum = 0x84DE;
  pub const GL_TEXTURE31: GLenum = 0x84DF;
  pub const GL_TEXTURE4: GLenum = 0x84C4;
  pub const GL_TEXTURE5: GLenum = 0x84C5;
  pub const GL_TEXTURE6: GLenum = 0x84C6;
  pub const GL_TEXTURE7: GLenum = 0x84C7;
  pub const GL_TEXTURE8: GLenum = 0x84C8;
  pub const GL_TEXTURE9: GLenum = 0x84C9;
  pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
  pub const GL_TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
  pub const GL_TEXTURE_3D: GLenum = 0x806F;
  pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
  pub const GL_TEXTURE_BINDING_2D: GLenum = 0x8069;
  pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
  pub const GL_TEXTURE_BINDING_3D: GLenum = 0x806A;
  pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
  pub const GL_TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
  pub const GL_TEXTURE_COMPARE_MODE: GLenum = 0x884C;
  pub const GL_TEXTURE_CUBE_MAP: GLenum = 0x8513;
  pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
  pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
  pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
  pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
  pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
  pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
  pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
  pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
  pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
  pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
  pub const GL_TEXTURE_MAX_LOD: GLenum = 0x813B;
  pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
  pub const GL_TEXTURE_MIN_LOD: GLenum = 0x813A;
  pub const GL_TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
  pub const GL_TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
  pub const GL_TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
  pub const GL_TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
  pub const GL_TEXTURE_WRAP_R: GLenum = 0x8072;
  pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
  pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
  pub const GL_TIMEOUT_EXPIRED: GLenum = 0x911B;
  pub const GL_TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
  pub const GL_TRANSFORM_FEEDBACK: GLenum = 0x8E22;
  pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
  pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
  pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
  pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
  pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
  pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
  pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
  pub const GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
  pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
  pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
  pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
  pub const GL_TRIANGLES: GLenum = 0x0004;
  pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
  pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
  pub const GL_TRUE: GLboolean = 1;
  pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
  pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
  pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
  pub const GL_UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
  pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
  pub const GL_UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
  pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
  pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
  pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
  pub const GL_UNIFORM_BUFFER: GLenum = 0x8A11;
  pub const GL_UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
  pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
  pub const GL_UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
  pub const GL_UNIFORM_BUFFER_START: GLenum = 0x8A29;
  pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
  pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
  pub const GL_UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
  pub const GL_UNIFORM_OFFSET: GLenum = 0x8A3B;
  pub const GL_UNIFORM_SIZE: GLenum = 0x8A38;
  pub const GL_UNIFORM_TYPE: GLenum = 0x8A37;
  pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5;
  pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
  pub const GL_UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
  pub const GL_UNPACK_SKIP_IMAGES: GLenum = 0x806D;
  pub const GL_UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
  pub const GL_UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
  pub const GL_UNSIGNALED: GLenum = 0x9118;
  pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
  pub const GL_UNSIGNED_INT: GLenum = 0x1405;
  pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
  pub const GL_UNSIGNED_INT_24_8: GLenum = 0x84FA;
  pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
  pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
  pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
  pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
  pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
  pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
  pub const GL_UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
  pub const GL_UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
  pub const GL_UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
  pub const GL_UNSIGNED_NORMALIZED: GLenum = 0x8C17;
  pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
  pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
  pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
  pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
  pub const GL_VALIDATE_STATUS: GLenum = 0x8B83;
  pub const GL_VENDOR: GLenum = 0x1F00;
  pub const GL_VERSION: GLenum = 0x1F02;
  pub const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
  pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
  pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
  pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
  pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
  pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
  pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
  pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
  pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
  pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
  pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
  pub const GL_VIEWPORT: GLenum = 0x0BA2;
  pub const GL_WAIT_FAILED: GLenum = 0x911D;
  pub const GL_ZERO: GLenum = 0;
}
pub use functions::*;
pub mod functions {
  use super::*;
  /// See [glActiveTexture](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glActiveTexture.xhtml)
  ///
  /// Fallbacks: ActiveTextureARB
  #[inline]
  pub unsafe fn glActiveTexture(texture: GLenum) {
    trace!("glActiveTexture");
    let p: *mut c_void = {
      let temp_p = storage::ActiveTexture.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glActiveTexture is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(texture);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glActiveTexture({:?}): {}", texture, err);
      }
    }
    out
  }
  /// See [glAttachShader](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glAttachShader.xhtml)
  ///
  /// Fallbacks: AttachObjectARB
  #[inline]
  pub unsafe fn glAttachShader(program: GLuint, shader: GLuint) {
    trace!("glAttachShader");
    let p: *mut c_void = {
      let temp_p = storage::AttachShader.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glAttachShader is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, GLuint)>(p)(
      program, shader,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glAttachShader({:?}, {:?}): {}", program, shader, err);
      }
    }
    out
  }
  /// See [glBeginQuery](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBeginQuery.xhtml)
  ///
  /// Fallbacks: BeginQueryARB
  #[inline]
  pub unsafe fn glBeginQuery(target: GLenum, id: GLuint) {
    trace!("glBeginQuery");
    let p: *mut c_void = {
      let temp_p = storage::BeginQuery.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBeginQuery is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLuint)>(p)(
      target, id,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBeginQuery({:?}, {:?}): {}", target, id, err);
      }
    }
    out
  }
  /// See [glBeginTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBeginTransformFeedback.xhtml)
  ///
  /// Fallbacks: BeginTransformFeedbackEXT, BeginTransformFeedbackNV
  #[inline]
  pub unsafe fn glBeginTransformFeedback(primitiveMode: GLenum) {
    trace!("glBeginTransformFeedback");
    let p: *mut c_void = {
      let temp_p = storage::BeginTransformFeedback.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBeginTransformFeedback is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(primitiveMode);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBeginTransformFeedback({:?}): {}", primitiveMode, err);
      }
    }
    out
  }
  /// See [glBindAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindAttribLocation.xhtml)
  ///
  /// Fallbacks: BindAttribLocationARB
  #[inline]
  pub unsafe fn glBindAttribLocation(
    program: GLuint,
    index: GLuint,
    name: *const GLchar,
  ) {
    trace!("glBindAttribLocation");
    let p: *mut c_void = {
      let temp_p = storage::BindAttribLocation.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindAttribLocation is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLuint, *const GLchar),
    >(p)(program, index, name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindAttribLocation({:?}, {:?}, {:?}): {}",
          program, index, name, err
        );
      }
    }
    out
  }
  /// See [glBindBuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindBuffer.xhtml)
  ///
  /// Fallbacks: BindBufferARB
  #[inline]
  pub unsafe fn glBindBuffer(target: GLenum, buffer: GLuint) {
    trace!("glBindBuffer");
    let p: *mut c_void = {
      let temp_p = storage::BindBuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindBuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLuint)>(p)(
      target, buffer,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindBuffer({:?}, {:?}): {}", target, buffer, err);
      }
    }
    out
  }
  /// See [glBindBufferBase](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindBufferBase.xhtml)
  ///
  /// Fallbacks: BindBufferBaseEXT, BindBufferBaseNV
  #[inline]
  pub unsafe fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) {
    trace!("glBindBufferBase");
    let p: *mut c_void = {
      let temp_p = storage::BindBufferBase.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindBufferBase is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLuint, GLuint),
    >(p)(target, index, buffer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindBufferBase({:?}, {:?}, {:?}): {}",
          target, index, buffer, err
        );
      }
    }
    out
  }
  /// See [glBindBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindBufferRange.xhtml)
  ///
  /// Fallbacks: BindBufferRangeEXT, BindBufferRangeNV
  #[inline]
  pub unsafe fn glBindBufferRange(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
  ) {
    trace!("glBindBufferRange");
    let p: *mut c_void = {
      let temp_p = storage::BindBufferRange.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindBufferRange is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr),
    >(p)(target, index, buffer, offset, size);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindBufferRange({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, index, buffer, offset, size, err
        );
      }
    }
    out
  }
  /// See [glBindFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindFramebuffer.xhtml)
  #[inline]
  pub unsafe fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    trace!("glBindFramebuffer");
    let p: *mut c_void = {
      let temp_p = storage::BindFramebuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindFramebuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLuint)>(p)(
      target,
      framebuffer,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindFramebuffer({:?}, {:?}): {}", target, framebuffer, err);
      }
    }
    out
  }
  /// See [glBindRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindRenderbuffer.xhtml)
  #[inline]
  pub unsafe fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    trace!("glBindRenderbuffer");
    let p: *mut c_void = {
      let temp_p = storage::BindRenderbuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindRenderbuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLuint)>(p)(
      target,
      renderbuffer,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindRenderbuffer({:?}, {:?}): {}", target, renderbuffer, err);
      }
    }
    out
  }
  /// See [glBindSampler](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindSampler.xhtml)
  #[inline]
  pub unsafe fn glBindSampler(unit: GLuint, sampler: GLuint) {
    trace!("glBindSampler");
    let p: *mut c_void = {
      let temp_p = storage::BindSampler.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindSampler is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, GLuint)>(p)(
      unit, sampler,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindSampler({:?}, {:?}): {}", unit, sampler, err);
      }
    }
    out
  }
  /// See [glBindTexture](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindTexture.xhtml)
  ///
  /// Fallbacks: BindTextureEXT
  #[inline]
  pub unsafe fn glBindTexture(target: GLenum, texture: GLuint) {
    trace!("glBindTexture");
    let p: *mut c_void = {
      let temp_p = storage::BindTexture.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindTexture is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLuint)>(p)(
      target, texture,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindTexture({:?}, {:?}): {}", target, texture, err);
      }
    }
    out
  }
  /// See [glBindTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindTransformFeedback.xhtml)
  #[inline]
  pub unsafe fn glBindTransformFeedback(target: GLenum, id: GLuint) {
    trace!("glBindTransformFeedback");
    let p: *mut c_void = {
      let temp_p = storage::BindTransformFeedback.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindTransformFeedback is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLuint)>(p)(
      target, id,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindTransformFeedback({:?}, {:?}): {}", target, id, err);
      }
    }
    out
  }
  /// See [glBindVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBindVertexArray.xhtml)
  ///
  /// Fallbacks: BindVertexArrayOES
  #[inline]
  pub unsafe fn glBindVertexArray(array: GLuint) {
    trace!("glBindVertexArray");
    let p: *mut c_void = {
      let temp_p = storage::BindVertexArray.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBindVertexArray is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(array);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBindVertexArray({:?}): {}", array, err);
      }
    }
    out
  }
  /// See [glBlendColor](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBlendColor.xhtml)
  ///
  /// Fallbacks: BlendColorEXT
  #[inline]
  pub unsafe fn glBlendColor(
    red: GLfloat,
    green: GLfloat,
    blue: GLfloat,
    alpha: GLfloat,
  ) {
    trace!("glBlendColor");
    let p: *mut c_void = {
      let temp_p = storage::BlendColor.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBlendColor is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat),
    >(p)(red, green, blue, alpha);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBlendColor({:?}, {:?}, {:?}, {:?}): {}",
          red, green, blue, alpha, err
        );
      }
    }
    out
  }
  /// See [glBlendEquation](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBlendEquation.xhtml)
  ///
  /// Fallbacks: BlendEquationEXT
  #[inline]
  pub unsafe fn glBlendEquation(mode: GLenum) {
    trace!("glBlendEquation");
    let p: *mut c_void = {
      let temp_p = storage::BlendEquation.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBlendEquation is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(mode);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBlendEquation({:?}): {}", mode, err);
      }
    }
    out
  }
  /// See [glBlendEquationSeparate](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBlendEquationSeparate.xhtml)
  ///
  /// Fallbacks: BlendEquationSeparateEXT
  #[inline]
  pub unsafe fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    trace!("glBlendEquationSeparate");
    let p: *mut c_void = {
      let temp_p = storage::BlendEquationSeparate.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBlendEquationSeparate is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLenum)>(p)(
      modeRGB, modeAlpha,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBlendEquationSeparate({:?}, {:?}): {}",
          modeRGB, modeAlpha, err
        );
      }
    }
    out
  }
  /// See [glBlendFunc](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBlendFunc.xhtml)
  #[inline]
  pub unsafe fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    trace!("glBlendFunc");
    let p: *mut c_void = {
      let temp_p = storage::BlendFunc.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBlendFunc is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLenum)>(p)(
      sfactor, dfactor,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBlendFunc({:?}, {:?}): {}", sfactor, dfactor, err);
      }
    }
    out
  }
  /// See [glBlendFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBlendFuncSeparate.xhtml)
  ///
  /// Fallbacks: BlendFuncSeparateEXT, BlendFuncSeparateINGR
  #[inline]
  pub unsafe fn glBlendFuncSeparate(
    sfactorRGB: GLenum,
    dfactorRGB: GLenum,
    sfactorAlpha: GLenum,
    dfactorAlpha: GLenum,
  ) {
    trace!("glBlendFuncSeparate");
    let p: *mut c_void = {
      let temp_p = storage::BlendFuncSeparate.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBlendFuncSeparate is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLenum, GLenum),
    >(p)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBlendFuncSeparate({:?}, {:?}, {:?}, {:?}): {}",
          sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha, err
        );
      }
    }
    out
  }
  /// See [glBlitFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBlitFramebuffer.xhtml)
  ///
  /// Fallbacks: BlitFramebufferEXT, BlitFramebufferNV
  #[inline]
  pub unsafe fn glBlitFramebuffer(
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
  ) {
    trace!("glBlitFramebuffer");
    let p: *mut c_void = {
      let temp_p = storage::BlitFramebuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBlitFramebuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLbitfield,
        GLenum,
      ),
    >(p)(
      srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBlitFramebuffer({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter, err);
      }
    }
    out
  }
  /// See [glBufferData](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBufferData.xhtml)
  ///
  /// Fallbacks: BufferDataARB
  #[inline]
  pub unsafe fn glBufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const c_void,
    usage: GLenum,
  ) {
    trace!("glBufferData");
    let p: *mut c_void = {
      let temp_p = storage::BufferData.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBufferData is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum),
    >(p)(target, size, data, usage);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBufferData({:?}, {:?}, {:?}, {:?}): {}",
          target, size, data, usage, err
        );
      }
    }
    out
  }
  /// See [glBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glBufferSubData.xhtml)
  ///
  /// Fallbacks: BufferSubDataARB
  #[inline]
  pub unsafe fn glBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
  ) {
    trace!("glBufferSubData");
    let p: *mut c_void = {
      let temp_p = storage::BufferSubData.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glBufferSubData is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void),
    >(p)(target, offset, size, data);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glBufferSubData({:?}, {:?}, {:?}, {:?}): {}",
          target, offset, size, data, err
        );
      }
    }
    out
  }
  /// See [glCheckFramebufferStatus](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCheckFramebufferStatus.xhtml)
  ///
  /// Fallbacks: CheckFramebufferStatusEXT
  #[inline]
  pub unsafe fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
    trace!("glCheckFramebufferStatus");
    let p: *mut c_void = {
      let temp_p = storage::CheckFramebufferStatus.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCheckFramebufferStatus is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<*mut c_void, extern "system" fn(GLenum) -> GLenum>(p)(target);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCheckFramebufferStatus({:?}): {}", target, err);
      }
    }
    out
  }
  /// See [glClear](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClear.xhtml)
  #[inline]
  pub unsafe fn glClear(mask: GLbitfield) {
    trace!("glClear");
    let p: *mut c_void = {
      let temp_p = storage::Clear.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClear is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLbitfield)>(p)(mask);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClear({:?}): {}", mask, err);
      }
    }
    out
  }
  /// See [glClearBufferfi](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClearBufferfi.xhtml)
  #[inline]
  pub unsafe fn glClearBufferfi(
    buffer: GLenum,
    drawbuffer: GLint,
    depth: GLfloat,
    stencil: GLint,
  ) {
    trace!("glClearBufferfi");
    let p: *mut c_void = {
      let temp_p = storage::ClearBufferfi.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClearBufferfi is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLint, GLfloat, GLint),
    >(p)(buffer, drawbuffer, depth, stencil);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClearBufferfi({:?}, {:?}, {:?}, {:?}): {}",
          buffer, drawbuffer, depth, stencil, err
        );
      }
    }
    out
  }
  /// See [glClearBuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClearBuffer.xhtml)
  #[inline]
  pub unsafe fn glClearBufferfv(
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLfloat,
  ) {
    trace!("glClearBufferfv");
    let p: *mut c_void = {
      let temp_p = storage::ClearBufferfv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClearBufferfv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLint, *const GLfloat),
    >(p)(buffer, drawbuffer, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClearBufferfv({:?}, {:?}, {:?}): {}",
          buffer, drawbuffer, value, err
        );
      }
    }
    out
  }
  /// See [glClearBuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClearBuffer.xhtml)
  #[inline]
  pub unsafe fn glClearBufferiv(
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLint,
  ) {
    trace!("glClearBufferiv");
    let p: *mut c_void = {
      let temp_p = storage::ClearBufferiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClearBufferiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLint, *const GLint),
    >(p)(buffer, drawbuffer, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClearBufferiv({:?}, {:?}, {:?}): {}",
          buffer, drawbuffer, value, err
        );
      }
    }
    out
  }
  /// See [glClearBuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClearBuffer.xhtml)
  #[inline]
  pub unsafe fn glClearBufferuiv(
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLuint,
  ) {
    trace!("glClearBufferuiv");
    let p: *mut c_void = {
      let temp_p = storage::ClearBufferuiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClearBufferuiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLint, *const GLuint),
    >(p)(buffer, drawbuffer, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClearBufferuiv({:?}, {:?}, {:?}): {}",
          buffer, drawbuffer, value, err
        );
      }
    }
    out
  }
  /// See [glClearColor](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClearColor.xhtml)
  #[inline]
  pub unsafe fn glClearColor(
    red: GLfloat,
    green: GLfloat,
    blue: GLfloat,
    alpha: GLfloat,
  ) {
    trace!("glClearColor");
    let p: *mut c_void = {
      let temp_p = storage::ClearColor.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClearColor is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat),
    >(p)(red, green, blue, alpha);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClearColor({:?}, {:?}, {:?}, {:?}): {}",
          red, green, blue, alpha, err
        );
      }
    }
    out
  }
  /// See [glClearDepthf](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClearDepthf.xhtml)
  ///
  /// Fallbacks: ClearDepthfOES
  #[inline]
  pub unsafe fn glClearDepthf(d: GLfloat) {
    trace!("glClearDepthf");
    let p: *mut c_void = {
      let temp_p = storage::ClearDepthf.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClearDepthf is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLfloat)>(p)(d);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClearDepthf({:?}): {}", d, err);
      }
    }
    out
  }
  /// See [glClearStencil](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClearStencil.xhtml)
  #[inline]
  pub unsafe fn glClearStencil(s: GLint) {
    trace!("glClearStencil");
    let p: *mut c_void = {
      let temp_p = storage::ClearStencil.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClearStencil is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLint)>(p)(s);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClearStencil({:?}): {}", s, err);
      }
    }
    out
  }
  /// See [glClientWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glClientWaitSync.xhtml)
  ///
  /// Fallbacks: ClientWaitSyncAPPLE
  #[inline]
  pub unsafe fn glClientWaitSync(
    sync: GLsync,
    flags: GLbitfield,
    timeout: GLuint64,
  ) -> GLenum {
    trace!("glClientWaitSync");
    let p: *mut c_void = {
      let temp_p = storage::ClientWaitSync.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glClientWaitSync is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum,
    >(p)(sync, flags, timeout);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glClientWaitSync({:?}, {:?}, {:?}): {}",
          sync, flags, timeout, err
        );
      }
    }
    out
  }
  /// See [glColorMask](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glColorMask.xhtml)
  #[inline]
  pub unsafe fn glColorMask(
    red: GLboolean,
    green: GLboolean,
    blue: GLboolean,
    alpha: GLboolean,
  ) {
    trace!("glColorMask");
    let p: *mut c_void = {
      let temp_p = storage::ColorMask.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glColorMask is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean),
    >(p)(red, green, blue, alpha);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glColorMask({:?}, {:?}, {:?}, {:?}): {}",
          red, green, blue, alpha, err
        );
      }
    }
    out
  }
  /// See [glCompileShader](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCompileShader.xhtml)
  ///
  /// Fallbacks: CompileShaderARB
  #[inline]
  pub unsafe fn glCompileShader(shader: GLuint) {
    trace!("glCompileShader");
    let p: *mut c_void = {
      let temp_p = storage::CompileShader.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCompileShader is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(shader);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCompileShader({:?}): {}", shader, err);
      }
    }
    out
  }
  /// See [glCompressedTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCompressedTexImage2D.xhtml)
  ///
  /// Fallbacks: CompressedTexImage2DARB
  #[inline]
  pub unsafe fn glCompressedTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  ) {
    trace!("glCompressedTexImage2D");
    let p: *mut c_void = {
      let temp_p = storage::CompressedTexImage2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCompressedTexImage2D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLenum,
        GLsizei,
        GLsizei,
        GLint,
        GLsizei,
        *const c_void,
      ),
    >(p)(
      target,
      level,
      internalformat,
      width,
      height,
      border,
      imageSize,
      data,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCompressedTexImage2D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, internalformat, width, height, border, imageSize, data, err);
      }
    }
    out
  }
  /// See [glCompressedTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCompressedTexImage3D.xhtml)
  ///
  /// Fallbacks: CompressedTexImage3DARB
  #[inline]
  pub unsafe fn glCompressedTexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  ) {
    trace!("glCompressedTexImage3D");
    let p: *mut c_void = {
      let temp_p = storage::CompressedTexImage3D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCompressedTexImage3D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLenum,
        GLsizei,
        GLsizei,
        GLsizei,
        GLint,
        GLsizei,
        *const c_void,
      ),
    >(p)(
      target,
      level,
      internalformat,
      width,
      height,
      depth,
      border,
      imageSize,
      data,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCompressedTexImage3D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, internalformat, width, height, depth, border, imageSize, data, err);
      }
    }
    out
  }
  /// See [glCompressedTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCompressedTexSubImage2D.xhtml)
  ///
  /// Fallbacks: CompressedTexSubImage2DARB
  #[inline]
  pub unsafe fn glCompressedTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  ) {
    trace!("glCompressedTexSubImage2D");
    let p: *mut c_void = {
      let temp_p = storage::CompressedTexSubImage2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCompressedTexSubImage2D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLenum,
        GLsizei,
        *const c_void,
      ),
    >(p)(
      target, level, xoffset, yoffset, width, height, format, imageSize, data,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCompressedTexSubImage2D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, xoffset, yoffset, width, height, format, imageSize, data, err);
      }
    }
    out
  }
  /// See [glCompressedTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCompressedTexSubImage3D.xhtml)
  ///
  /// Fallbacks: CompressedTexSubImage3DARB
  #[inline]
  pub unsafe fn glCompressedTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  ) {
    trace!("glCompressedTexSubImage3D");
    let p: *mut c_void = {
      let temp_p = storage::CompressedTexSubImage3D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCompressedTexSubImage3D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLsizei,
        *const c_void,
      ),
    >(p)(
      target, level, xoffset, yoffset, zoffset, width, height, depth, format,
      imageSize, data,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCompressedTexSubImage3D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data, err);
      }
    }
    out
  }
  /// See [glCopyBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCopyBufferSubData.xhtml)
  ///
  /// Fallbacks: CopyBufferSubDataNV
  #[inline]
  pub unsafe fn glCopyBufferSubData(
    readTarget: GLenum,
    writeTarget: GLenum,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
  ) {
    trace!("glCopyBufferSubData");
    let p: *mut c_void = {
      let temp_p = storage::CopyBufferSubData.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCopyBufferSubData is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr),
    >(p)(readTarget, writeTarget, readOffset, writeOffset, size);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCopyBufferSubData({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          readTarget, writeTarget, readOffset, writeOffset, size, err
        );
      }
    }
    out
  }
  /// See [glCopyTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCopyTexImage2D.xhtml)
  ///
  /// Fallbacks: CopyTexImage2DEXT
  #[inline]
  pub unsafe fn glCopyTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
  ) {
    trace!("glCopyTexImage2D");
    let p: *mut c_void = {
      let temp_p = storage::CopyTexImage2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCopyTexImage2D is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<
        *mut c_void,
        extern "system" fn(
          GLenum,
          GLint,
          GLenum,
          GLint,
          GLint,
          GLsizei,
          GLsizei,
          GLint,
        ),
      >(p)(target, level, internalformat, x, y, width, height, border);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCopyTexImage2D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, level, internalformat, x, y, width, height, border, err
        );
      }
    }
    out
  }
  /// See [glCopyTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCopyTexSubImage2D.xhtml)
  ///
  /// Fallbacks: CopyTexSubImage2DEXT
  #[inline]
  pub unsafe fn glCopyTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  ) {
    trace!("glCopyTexSubImage2D");
    let p: *mut c_void = {
      let temp_p = storage::CopyTexSubImage2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCopyTexSubImage2D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
      ),
    >(p)(target, level, xoffset, yoffset, x, y, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCopyTexSubImage2D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, xoffset, yoffset, x, y, width, height, err);
      }
    }
    out
  }
  /// See [glCopyTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCopyTexSubImage3D.xhtml)
  ///
  /// Fallbacks: CopyTexSubImage3DEXT
  #[inline]
  pub unsafe fn glCopyTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  ) {
    trace!("glCopyTexSubImage3D");
    let p: *mut c_void = {
      let temp_p = storage::CopyTexSubImage3D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCopyTexSubImage3D is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<
        *mut c_void,
        extern "system" fn(
          GLenum,
          GLint,
          GLint,
          GLint,
          GLint,
          GLint,
          GLint,
          GLsizei,
          GLsizei,
        ),
      >(p)(target, level, xoffset, yoffset, zoffset, x, y, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCopyTexSubImage3D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, xoffset, yoffset, zoffset, x, y, width, height, err);
      }
    }
    out
  }
  /// See [glCreateProgram](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCreateProgram.xhtml)
  ///
  /// Fallbacks: CreateProgramObjectARB
  #[inline]
  pub unsafe fn glCreateProgram() -> GLuint {
    trace!("glCreateProgram");
    let p: *mut c_void = {
      let temp_p = storage::CreateProgram.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCreateProgram is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn() -> GLuint>(p)();
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCreateProgram(): {}", err);
      }
    }
    out
  }
  /// See [glCreateShader](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCreateShader.xhtml)
  ///
  /// Fallbacks: CreateShaderObjectARB
  #[inline]
  pub unsafe fn glCreateShader(type_: GLenum) -> GLuint {
    trace!("glCreateShader");
    let p: *mut c_void = {
      let temp_p = storage::CreateShader.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCreateShader is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<*mut c_void, extern "system" fn(GLenum) -> GLuint>(p)(type_);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCreateShader({:?}): {}", type_, err);
      }
    }
    out
  }
  /// See [glCullFace](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glCullFace.xhtml)
  #[inline]
  pub unsafe fn glCullFace(mode: GLenum) {
    trace!("glCullFace");
    let p: *mut c_void = {
      let temp_p = storage::CullFace.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glCullFace is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(mode);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glCullFace({:?}): {}", mode, err);
      }
    }
    out
  }
  /// See [glDeleteBuffers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteBuffers.xhtml)
  ///
  /// Fallbacks: DeleteBuffersARB
  #[inline]
  pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    trace!("glDeleteBuffers");
    let p: *mut c_void = {
      let temp_p = storage::DeleteBuffers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteBuffers is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(n, buffers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteBuffers({:?}, {:?}): {}", n, buffers, err);
      }
    }
    out
  }
  /// See [glDeleteFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteFramebuffers.xhtml)
  ///
  /// Fallbacks: DeleteFramebuffersEXT
  #[inline]
  pub unsafe fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    trace!("glDeleteFramebuffers");
    let p: *mut c_void = {
      let temp_p = storage::DeleteFramebuffers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteFramebuffers is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(n, framebuffers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteFramebuffers({:?}, {:?}): {}", n, framebuffers, err);
      }
    }
    out
  }
  /// See [glDeleteProgram](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteProgram.xhtml)
  #[inline]
  pub unsafe fn glDeleteProgram(program: GLuint) {
    trace!("glDeleteProgram");
    let p: *mut c_void = {
      let temp_p = storage::DeleteProgram.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteProgram is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(program);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteProgram({:?}): {}", program, err);
      }
    }
    out
  }
  /// See [glDeleteQueries](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteQueries.xhtml)
  ///
  /// Fallbacks: DeleteQueriesARB
  #[inline]
  pub unsafe fn glDeleteQueries(n: GLsizei, ids: *const GLuint) {
    trace!("glDeleteQueries");
    let p: *mut c_void = {
      let temp_p = storage::DeleteQueries.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteQueries is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(n, ids);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteQueries({:?}, {:?}): {}", n, ids, err);
      }
    }
    out
  }
  /// See [glDeleteRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteRenderbuffers.xhtml)
  ///
  /// Fallbacks: DeleteRenderbuffersEXT
  #[inline]
  pub unsafe fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    trace!("glDeleteRenderbuffers");
    let p: *mut c_void = {
      let temp_p = storage::DeleteRenderbuffers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteRenderbuffers is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(n, renderbuffers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteRenderbuffers({:?}, {:?}): {}", n, renderbuffers, err);
      }
    }
    out
  }
  /// See [glDeleteSamplers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteSamplers.xhtml)
  #[inline]
  pub unsafe fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint) {
    trace!("glDeleteSamplers");
    let p: *mut c_void = {
      let temp_p = storage::DeleteSamplers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteSamplers is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(count, samplers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteSamplers({:?}, {:?}): {}", count, samplers, err);
      }
    }
    out
  }
  /// See [glDeleteShader](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteShader.xhtml)
  #[inline]
  pub unsafe fn glDeleteShader(shader: GLuint) {
    trace!("glDeleteShader");
    let p: *mut c_void = {
      let temp_p = storage::DeleteShader.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteShader is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(shader);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteShader({:?}): {}", shader, err);
      }
    }
    out
  }
  /// See [glDeleteSync](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteSync.xhtml)
  ///
  /// Fallbacks: DeleteSyncAPPLE
  #[inline]
  pub unsafe fn glDeleteSync(sync: GLsync) {
    trace!("glDeleteSync");
    let p: *mut c_void = {
      let temp_p = storage::DeleteSync.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteSync is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsync)>(p)(sync);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteSync({:?}): {}", sync, err);
      }
    }
    out
  }
  /// See [glDeleteTextures](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteTextures.xhtml)
  #[inline]
  pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    trace!("glDeleteTextures");
    let p: *mut c_void = {
      let temp_p = storage::DeleteTextures.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteTextures is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(n, textures);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteTextures({:?}, {:?}): {}", n, textures, err);
      }
    }
    out
  }
  /// See [glDeleteTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteTransformFeedbacks.xhtml)
  ///
  /// Fallbacks: DeleteTransformFeedbacksNV
  #[inline]
  pub unsafe fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) {
    trace!("glDeleteTransformFeedbacks");
    let p: *mut c_void = {
      let temp_p = storage::DeleteTransformFeedbacks.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteTransformFeedbacks is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(n, ids);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteTransformFeedbacks({:?}, {:?}): {}", n, ids, err);
      }
    }
    out
  }
  /// See [glDeleteVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDeleteVertexArrays.xhtml)
  ///
  /// Fallbacks: DeleteVertexArraysAPPLE, DeleteVertexArraysOES
  #[inline]
  pub unsafe fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
    trace!("glDeleteVertexArrays");
    let p: *mut c_void = {
      let temp_p = storage::DeleteVertexArrays.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDeleteVertexArrays is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLuint),
    >(p)(n, arrays);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDeleteVertexArrays({:?}, {:?}): {}", n, arrays, err);
      }
    }
    out
  }
  /// See [glDepthFunc](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDepthFunc.xhtml)
  #[inline]
  pub unsafe fn glDepthFunc(func: GLenum) {
    trace!("glDepthFunc");
    let p: *mut c_void = {
      let temp_p = storage::DepthFunc.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDepthFunc is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(func);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDepthFunc({:?}): {}", func, err);
      }
    }
    out
  }
  /// See [glDepthMask](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDepthMask.xhtml)
  #[inline]
  pub unsafe fn glDepthMask(flag: GLboolean) {
    trace!("glDepthMask");
    let p: *mut c_void = {
      let temp_p = storage::DepthMask.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDepthMask is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLboolean)>(p)(flag);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDepthMask({:?}): {}", flag, err);
      }
    }
    out
  }
  /// See [glDepthRangef](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDepthRangef.xhtml)
  ///
  /// Fallbacks: DepthRangefOES
  #[inline]
  pub unsafe fn glDepthRangef(n: GLfloat, f: GLfloat) {
    trace!("glDepthRangef");
    let p: *mut c_void = {
      let temp_p = storage::DepthRangef.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDepthRangef is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<*mut c_void, extern "system" fn(GLfloat, GLfloat)>(p)(n, f);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDepthRangef({:?}, {:?}): {}", n, f, err);
      }
    }
    out
  }
  /// See [glDetachShader](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDetachShader.xhtml)
  ///
  /// Fallbacks: DetachObjectARB
  #[inline]
  pub unsafe fn glDetachShader(program: GLuint, shader: GLuint) {
    trace!("glDetachShader");
    let p: *mut c_void = {
      let temp_p = storage::DetachShader.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDetachShader is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, GLuint)>(p)(
      program, shader,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDetachShader({:?}, {:?}): {}", program, shader, err);
      }
    }
    out
  }
  /// See [glDisable](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDisable.xhtml)
  #[inline]
  pub unsafe fn glDisable(cap: GLenum) {
    trace!("glDisable");
    let p: *mut c_void = {
      let temp_p = storage::Disable.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDisable is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(cap);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDisable({:?}): {}", cap, err);
      }
    }
    out
  }
  /// See [glDisableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDisableVertexAttribArray.xhtml)
  ///
  /// Fallbacks: DisableVertexAttribArrayARB
  #[inline]
  pub unsafe fn glDisableVertexAttribArray(index: GLuint) {
    trace!("glDisableVertexAttribArray");
    let p: *mut c_void = {
      let temp_p = storage::DisableVertexAttribArray.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDisableVertexAttribArray is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(index);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDisableVertexAttribArray({:?}): {}", index, err);
      }
    }
    out
  }
  /// See [glDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDrawArrays.xhtml)
  ///
  /// Fallbacks: DrawArraysEXT
  #[inline]
  pub unsafe fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    trace!("glDrawArrays");
    let p: *mut c_void = {
      let temp_p = storage::DrawArrays.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDrawArrays is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLint, GLsizei),
    >(p)(mode, first, count);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDrawArrays({:?}, {:?}, {:?}): {}", mode, first, count, err);
      }
    }
    out
  }
  /// See [glDrawArraysInstanced](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDrawArraysInstanced.xhtml)
  ///
  /// Fallbacks: DrawArraysInstancedANGLE, DrawArraysInstancedARB,
  /// DrawArraysInstancedEXT, DrawArraysInstancedNV
  #[inline]
  pub unsafe fn glDrawArraysInstanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
  ) {
    trace!("glDrawArraysInstanced");
    let p: *mut c_void = {
      let temp_p = storage::DrawArraysInstanced.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDrawArraysInstanced is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLint, GLsizei, GLsizei),
    >(p)(mode, first, count, instancecount);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDrawArraysInstanced({:?}, {:?}, {:?}, {:?}): {}",
          mode, first, count, instancecount, err
        );
      }
    }
    out
  }
  /// See [glDrawBuffers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDrawBuffers.xhtml)
  ///
  /// Fallbacks: DrawBuffersARB, DrawBuffersATI, DrawBuffersEXT
  #[inline]
  pub unsafe fn glDrawBuffers(n: GLsizei, bufs: *const GLenum) {
    trace!("glDrawBuffers");
    let p: *mut c_void = {
      let temp_p = storage::DrawBuffers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDrawBuffers is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsizei, *const GLenum),
    >(p)(n, bufs);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDrawBuffers({:?}, {:?}): {}", n, bufs, err);
      }
    }
    out
  }
  /// See [glDrawElements](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDrawElements.xhtml)
  #[inline]
  pub unsafe fn glDrawElements(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
  ) {
    trace!("glDrawElements");
    let p: *mut c_void = {
      let temp_p = storage::DrawElements.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDrawElements is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLsizei, GLenum, *const c_void),
    >(p)(mode, count, type_, indices);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDrawElements({:?}, {:?}, {:?}, {:?}): {}",
          mode, count, type_, indices, err
        );
      }
    }
    out
  }
  /// See [glDrawElementsInstanced](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDrawElementsInstanced.xhtml)
  ///
  /// Fallbacks: DrawElementsInstancedANGLE, DrawElementsInstancedARB,
  /// DrawElementsInstancedEXT, DrawElementsInstancedNV
  #[inline]
  pub unsafe fn glDrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
  ) {
    trace!("glDrawElementsInstanced");
    let p: *mut c_void = {
      let temp_p = storage::DrawElementsInstanced.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDrawElementsInstanced is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei),
    >(p)(mode, count, type_, indices, instancecount);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDrawElementsInstanced({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          mode, count, type_, indices, instancecount, err
        );
      }
    }
    out
  }
  /// See [glDrawRangeElements](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDrawRangeElements.xhtml)
  ///
  /// Fallbacks: DrawRangeElementsEXT
  #[inline]
  pub unsafe fn glDrawRangeElements(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
  ) {
    trace!("glDrawRangeElements");
    let p: *mut c_void = {
      let temp_p = storage::DrawRangeElements.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glDrawRangeElements is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLuint,
        GLuint,
        GLsizei,
        GLenum,
        *const c_void,
      ),
    >(p)(mode, start, end, count, type_, indices);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glDrawRangeElements({:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}",
          mode, start, end, count, type_, indices, err
        );
      }
    }
    out
  }
  /// See [glEnable](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glEnable.xhtml)
  #[inline]
  pub unsafe fn glEnable(cap: GLenum) {
    trace!("glEnable");
    let p: *mut c_void = {
      let temp_p = storage::Enable.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glEnable is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(cap);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glEnable({:?}): {}", cap, err);
      }
    }
    out
  }
  /// See [glEnableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glEnableVertexAttribArray.xhtml)
  ///
  /// Fallbacks: EnableVertexAttribArrayARB
  #[inline]
  pub unsafe fn glEnableVertexAttribArray(index: GLuint) {
    trace!("glEnableVertexAttribArray");
    let p: *mut c_void = {
      let temp_p = storage::EnableVertexAttribArray.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glEnableVertexAttribArray is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(index);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glEnableVertexAttribArray({:?}): {}", index, err);
      }
    }
    out
  }
  /// See [glEndQuery](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glEndQuery.xhtml)
  ///
  /// Fallbacks: EndQueryARB
  #[inline]
  pub unsafe fn glEndQuery(target: GLenum) {
    trace!("glEndQuery");
    let p: *mut c_void = {
      let temp_p = storage::EndQuery.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glEndQuery is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(target);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glEndQuery({:?}): {}", target, err);
      }
    }
    out
  }
  /// See [glEndTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glEndTransformFeedback.xhtml)
  ///
  /// Fallbacks: EndTransformFeedbackEXT, EndTransformFeedbackNV
  #[inline]
  pub unsafe fn glEndTransformFeedback() {
    trace!("glEndTransformFeedback");
    let p: *mut c_void = {
      let temp_p = storage::EndTransformFeedback.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glEndTransformFeedback is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn()>(p)();
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glEndTransformFeedback(): {}", err);
      }
    }
    out
  }
  /// See [glFenceSync](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFenceSync.xhtml)
  ///
  /// Fallbacks: FenceSyncAPPLE
  #[inline]
  pub unsafe fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync {
    trace!("glFenceSync");
    let p: *mut c_void = {
      let temp_p = storage::FenceSync.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFenceSync is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLbitfield) -> GLsync,
    >(p)(condition, flags);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFenceSync({:?}, {:?}): {}", condition, flags, err);
      }
    }
    out
  }
  /// See [glFinish](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFinish.xhtml)
  #[inline]
  pub unsafe fn glFinish() {
    trace!("glFinish");
    let p: *mut c_void = {
      let temp_p = storage::Finish.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFinish is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn()>(p)();
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFinish(): {}", err);
      }
    }
    out
  }
  /// See [glFlush](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFlush.xhtml)
  #[inline]
  pub unsafe fn glFlush() {
    trace!("glFlush");
    let p: *mut c_void = {
      let temp_p = storage::Flush.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFlush is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn()>(p)();
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFlush(): {}", err);
      }
    }
    out
  }
  /// See [glFlushMappedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFlushMappedBufferRange.xhtml)
  ///
  /// Fallbacks: FlushMappedBufferRangeAPPLE, FlushMappedBufferRangeEXT
  #[inline]
  pub unsafe fn glFlushMappedBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
  ) {
    trace!("glFlushMappedBufferRange");
    let p: *mut c_void = {
      let temp_p = storage::FlushMappedBufferRange.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFlushMappedBufferRange is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLintptr, GLsizeiptr),
    >(p)(target, offset, length);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFlushMappedBufferRange({:?}, {:?}, {:?}): {}",
          target, offset, length, err
        );
      }
    }
    out
  }
  /// See [glFramebufferRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFramebufferRenderbuffer.xhtml)
  ///
  /// Fallbacks: FramebufferRenderbufferEXT
  #[inline]
  pub unsafe fn glFramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
  ) {
    trace!("glFramebufferRenderbuffer");
    let p: *mut c_void = {
      let temp_p = storage::FramebufferRenderbuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFramebufferRenderbuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLenum, GLuint),
    >(p)(target, attachment, renderbuffertarget, renderbuffer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFramebufferRenderbuffer({:?}, {:?}, {:?}, {:?}): {}",
          target, attachment, renderbuffertarget, renderbuffer, err
        );
      }
    }
    out
  }
  /// See [glFramebufferTexture2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFramebufferTexture2D.xhtml)
  ///
  /// Fallbacks: FramebufferTexture2DEXT
  #[inline]
  pub unsafe fn glFramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
  ) {
    trace!("glFramebufferTexture2D");
    let p: *mut c_void = {
      let temp_p = storage::FramebufferTexture2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFramebufferTexture2D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint),
    >(p)(target, attachment, textarget, texture, level);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFramebufferTexture2D({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, attachment, textarget, texture, level, err
        );
      }
    }
    out
  }
  /// See [glFramebufferTextureLayer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFramebufferTextureLayer.xhtml)
  ///
  /// Fallbacks: FramebufferTextureLayerARB, FramebufferTextureLayerEXT
  #[inline]
  pub unsafe fn glFramebufferTextureLayer(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
  ) {
    trace!("glFramebufferTextureLayer");
    let p: *mut c_void = {
      let temp_p = storage::FramebufferTextureLayer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFramebufferTextureLayer is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint),
    >(p)(target, attachment, texture, level, layer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFramebufferTextureLayer({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, attachment, texture, level, layer, err
        );
      }
    }
    out
  }
  /// See [glFrontFace](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glFrontFace.xhtml)
  #[inline]
  pub unsafe fn glFrontFace(mode: GLenum) {
    trace!("glFrontFace");
    let p: *mut c_void = {
      let temp_p = storage::FrontFace.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glFrontFace is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(mode);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glFrontFace({:?}): {}", mode, err);
      }
    }
    out
  }
  /// See [glGenBuffers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenBuffers.xhtml)
  ///
  /// Fallbacks: GenBuffersARB
  #[inline]
  pub unsafe fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    trace!("glGenBuffers");
    let p: *mut c_void = {
      let temp_p = storage::GenBuffers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenBuffers is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(n, buffers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenBuffers({:?}, {:?}): {}", n, buffers, err);
      }
    }
    out
  }
  /// See [glGenFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenFramebuffers.xhtml)
  ///
  /// Fallbacks: GenFramebuffersEXT
  #[inline]
  pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    trace!("glGenFramebuffers");
    let p: *mut c_void = {
      let temp_p = storage::GenFramebuffers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenFramebuffers is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(n, framebuffers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenFramebuffers({:?}, {:?}): {}", n, framebuffers, err);
      }
    }
    out
  }
  /// See [glGenQueries](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenQueries.xhtml)
  ///
  /// Fallbacks: GenQueriesARB
  #[inline]
  pub unsafe fn glGenQueries(n: GLsizei, ids: *mut GLuint) {
    trace!("glGenQueries");
    let p: *mut c_void = {
      let temp_p = storage::GenQueries.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenQueries is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(n, ids);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenQueries({:?}, {:?}): {}", n, ids, err);
      }
    }
    out
  }
  /// See [glGenRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenRenderbuffers.xhtml)
  ///
  /// Fallbacks: GenRenderbuffersEXT
  #[inline]
  pub unsafe fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    trace!("glGenRenderbuffers");
    let p: *mut c_void = {
      let temp_p = storage::GenRenderbuffers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenRenderbuffers is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(n, renderbuffers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenRenderbuffers({:?}, {:?}): {}", n, renderbuffers, err);
      }
    }
    out
  }
  /// See [glGenSamplers](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenSamplers.xhtml)
  #[inline]
  pub unsafe fn glGenSamplers(count: GLsizei, samplers: *mut GLuint) {
    trace!("glGenSamplers");
    let p: *mut c_void = {
      let temp_p = storage::GenSamplers.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenSamplers is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(count, samplers);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenSamplers({:?}, {:?}): {}", count, samplers, err);
      }
    }
    out
  }
  /// See [glGenTextures](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenTextures.xhtml)
  #[inline]
  pub unsafe fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    trace!("glGenTextures");
    let p: *mut c_void = {
      let temp_p = storage::GenTextures.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenTextures is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(n, textures);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenTextures({:?}, {:?}): {}", n, textures, err);
      }
    }
    out
  }
  /// See [glGenTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenTransformFeedbacks.xhtml)
  ///
  /// Fallbacks: GenTransformFeedbacksNV
  #[inline]
  pub unsafe fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    trace!("glGenTransformFeedbacks");
    let p: *mut c_void = {
      let temp_p = storage::GenTransformFeedbacks.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenTransformFeedbacks is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(n, ids);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenTransformFeedbacks({:?}, {:?}): {}", n, ids, err);
      }
    }
    out
  }
  /// See [glGenVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenVertexArrays.xhtml)
  ///
  /// Fallbacks: GenVertexArraysAPPLE, GenVertexArraysOES
  #[inline]
  pub unsafe fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    trace!("glGenVertexArrays");
    let p: *mut c_void = {
      let temp_p = storage::GenVertexArrays.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenVertexArrays is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsizei, *mut GLuint)>(
      p,
    )(n, arrays);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenVertexArrays({:?}, {:?}): {}", n, arrays, err);
      }
    }
    out
  }
  /// See [glGenerateMipmap](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGenerateMipmap.xhtml)
  ///
  /// Fallbacks: GenerateMipmapEXT
  #[inline]
  pub unsafe fn glGenerateMipmap(target: GLenum) {
    trace!("glGenerateMipmap");
    let p: *mut c_void = {
      let temp_p = storage::GenerateMipmap.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGenerateMipmap is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(target);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGenerateMipmap({:?}): {}", target, err);
      }
    }
    out
  }
  /// See [glGetActiveAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetActiveAttrib.xhtml)
  ///
  /// Fallbacks: GetActiveAttribARB
  #[inline]
  pub unsafe fn glGetActiveAttrib(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
  ) {
    trace!("glGetActiveAttrib");
    let p: *mut c_void = {
      let temp_p = storage::GetActiveAttrib.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetActiveAttrib is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLuint,
        GLuint,
        GLsizei,
        *mut GLsizei,
        *mut GLint,
        *mut GLenum,
        *mut GLchar,
      ),
    >(p)(program, index, bufSize, length, size, type_, name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetActiveAttrib({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}",
          program, index, bufSize, length, size, type_, name, err
        );
      }
    }
    out
  }
  /// See [glGetActiveUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetActiveUniform.xhtml)
  ///
  /// Fallbacks: GetActiveUniformARB
  #[inline]
  pub unsafe fn glGetActiveUniform(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
  ) {
    trace!("glGetActiveUniform");
    let p: *mut c_void = {
      let temp_p = storage::GetActiveUniform.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetActiveUniform is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLuint,
        GLuint,
        GLsizei,
        *mut GLsizei,
        *mut GLint,
        *mut GLenum,
        *mut GLchar,
      ),
    >(p)(program, index, bufSize, length, size, type_, name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetActiveUniform({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}",
          program, index, bufSize, length, size, type_, name, err
        );
      }
    }
    out
  }
  /// See [glGetActiveUniformBlockName](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetActiveUniformBlockName.xhtml)
  #[inline]
  pub unsafe fn glGetActiveUniformBlockName(
    program: GLuint,
    uniformBlockIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformBlockName: *mut GLchar,
  ) {
    trace!("glGetActiveUniformBlockName");
    let p: *mut c_void = {
      let temp_p = storage::GetActiveUniformBlockName.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetActiveUniformBlockName is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<
        *mut c_void,
        extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar),
      >(p)(program, uniformBlockIndex, bufSize, length, uniformBlockName);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetActiveUniformBlockName({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          program, uniformBlockIndex, bufSize, length, uniformBlockName, err
        );
      }
    }
    out
  }
  /// See [glGetActiveUniformBlockiv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetActiveUniformBlockiv.xhtml)
  #[inline]
  pub unsafe fn glGetActiveUniformBlockiv(
    program: GLuint,
    uniformBlockIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetActiveUniformBlockiv");
    let p: *mut c_void = {
      let temp_p = storage::GetActiveUniformBlockiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetActiveUniformBlockiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLuint, GLenum, *mut GLint),
    >(p)(program, uniformBlockIndex, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetActiveUniformBlockiv({:?}, {:?}, {:?}, {:?}): {}",
          program, uniformBlockIndex, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetActiveUniformsiv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetActiveUniformsiv.xhtml)
  #[inline]
  pub unsafe fn glGetActiveUniformsiv(
    program: GLuint,
    uniformCount: GLsizei,
    uniformIndices: *const GLuint,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetActiveUniformsiv");
    let p: *mut c_void = {
      let temp_p = storage::GetActiveUniformsiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetActiveUniformsiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint),
    >(p)(program, uniformCount, uniformIndices, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetActiveUniformsiv({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          program, uniformCount, uniformIndices, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetAttachedShaders](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetAttachedShaders.xhtml)
  #[inline]
  pub unsafe fn glGetAttachedShaders(
    program: GLuint,
    maxCount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
  ) {
    trace!("glGetAttachedShaders");
    let p: *mut c_void = {
      let temp_p = storage::GetAttachedShaders.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetAttachedShaders is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint),
    >(p)(program, maxCount, count, shaders);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetAttachedShaders({:?}, {:?}, {:?}, {:?}): {}",
          program, maxCount, count, shaders, err
        );
      }
    }
    out
  }
  /// See [glGetAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetAttribLocation.xhtml)
  ///
  /// Fallbacks: GetAttribLocationARB
  #[inline]
  pub unsafe fn glGetAttribLocation(
    program: GLuint,
    name: *const GLchar,
  ) -> GLint {
    trace!("glGetAttribLocation");
    let p: *mut c_void = {
      let temp_p = storage::GetAttribLocation.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetAttribLocation is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLchar) -> GLint,
    >(p)(program, name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetAttribLocation({:?}, {:?}): {}", program, name, err);
      }
    }
    out
  }
  /// See [glGet](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGet.xhtml)
  #[inline]
  pub unsafe fn glGetBooleanv(pname: GLenum, data: *mut GLboolean) {
    trace!("glGetBooleanv");
    let p: *mut c_void = {
      let temp_p = storage::GetBooleanv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetBooleanv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, *mut GLboolean),
    >(p)(pname, data);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetBooleanv({:?}, {:?}): {}", pname, data, err);
      }
    }
    out
  }
  /// See [glGetBufferParameteri64v](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetBufferParameter.xhtml)
  #[inline]
  pub unsafe fn glGetBufferParameteri64v(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint64,
  ) {
    trace!("glGetBufferParameteri64v");
    let p: *mut c_void = {
      let temp_p = storage::GetBufferParameteri64v.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetBufferParameteri64v is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *mut GLint64),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetBufferParameteri64v({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetBufferParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetBufferParameter.xhtml)
  ///
  /// Fallbacks: GetBufferParameterivARB
  #[inline]
  pub unsafe fn glGetBufferParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetBufferParameteriv");
    let p: *mut c_void = {
      let temp_p = storage::GetBufferParameteriv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetBufferParameteriv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *mut GLint),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetBufferParameteriv({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetBufferPointerv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetBufferPointerv.xhtml)
  ///
  /// Fallbacks: GetBufferPointervARB, GetBufferPointervOES
  #[inline]
  pub unsafe fn glGetBufferPointerv(
    target: GLenum,
    pname: GLenum,
    params: *const *mut c_void,
  ) {
    trace!("glGetBufferPointerv");
    let p: *mut c_void = {
      let temp_p = storage::GetBufferPointerv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetBufferPointerv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *const *mut c_void),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetBufferPointerv({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetError](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetError.xhtml)
  #[inline]
  pub unsafe fn glGetError() -> GLenum {
    trace!("glGetError");
    let p: *mut c_void = {
      let temp_p = storage::GetError.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetError is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn() -> GLenum>(p)();
    out
  }
  /// See [glGet](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGet.xhtml)
  #[inline]
  pub unsafe fn glGetFloatv(pname: GLenum, data: *mut GLfloat) {
    trace!("glGetFloatv");
    let p: *mut c_void = {
      let temp_p = storage::GetFloatv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetFloatv is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, *mut GLfloat)>(
      p,
    )(pname, data);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetFloatv({:?}, {:?}): {}", pname, data, err);
      }
    }
    out
  }
  /// See [glGetFragDataLocation](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetFragDataLocation.xhtml)
  ///
  /// Fallbacks: GetFragDataLocationEXT
  #[inline]
  pub unsafe fn glGetFragDataLocation(
    program: GLuint,
    name: *const GLchar,
  ) -> GLint {
    trace!("glGetFragDataLocation");
    let p: *mut c_void = {
      let temp_p = storage::GetFragDataLocation.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetFragDataLocation is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLchar) -> GLint,
    >(p)(program, name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetFragDataLocation({:?}, {:?}): {}", program, name, err);
      }
    }
    out
  }
  /// See [glGetFramebufferAttachmentParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetFramebufferAttachmentParameteriv.xhtml)
  ///
  /// Fallbacks: GetFramebufferAttachmentParameterivEXT
  #[inline]
  pub unsafe fn glGetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetFramebufferAttachmentParameteriv");
    let p: *mut c_void = {
      let temp_p =
        storage::GetFramebufferAttachmentParameteriv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetFramebufferAttachmentParameteriv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLenum, *mut GLint),
    >(p)(target, attachment, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetFramebufferAttachmentParameteriv({:?}, {:?}, {:?}, {:?}): {}",
          target, attachment, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGet](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGet.xhtml)
  #[inline]
  pub unsafe fn glGetInteger64i_v(
    target: GLenum,
    index: GLuint,
    data: *mut GLint64,
  ) {
    trace!("glGetInteger64i_v");
    let p: *mut c_void = {
      let temp_p = storage::GetInteger64i_v.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetInteger64i_v is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLuint, *mut GLint64),
    >(p)(target, index, data);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!(
          "GetInteger64i_v({:?}, {:?}, {:?}): {}",
          target, index, data, err
        );
      }
    }
    out
  }
  /// See [glGet](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGet.xhtml)
  ///
  /// Fallbacks: GetInteger64vAPPLE
  #[inline]
  pub unsafe fn glGetInteger64v(pname: GLenum, data: *mut GLint64) {
    trace!("glGetInteger64v");
    let p: *mut c_void = {
      let temp_p = storage::GetInteger64v.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetInteger64v is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, *mut GLint64)>(
      p,
    )(pname, data);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetInteger64v({:?}, {:?}): {}", pname, data, err);
      }
    }
    out
  }
  /// See [glGetInteger](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetInteger.xhtml)
  ///
  /// Fallbacks: GetIntegerIndexedvEXT
  #[inline]
  pub unsafe fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) {
    trace!("glGetIntegeri_v");
    let p: *mut c_void = {
      let temp_p = storage::GetIntegeri_v.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetIntegeri_v is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLuint, *mut GLint),
    >(p)(target, index, data);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("GetIntegeri_v({:?}, {:?}, {:?}): {}", target, index, data, err);
      }
    }
    out
  }
  /// See [glGet](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGet.xhtml)
  #[inline]
  pub unsafe fn glGetIntegerv(pname: GLenum, data: *mut GLint) {
    trace!("glGetIntegerv");
    let p: *mut c_void = {
      let temp_p = storage::GetIntegerv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetIntegerv is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, *mut GLint)>(
      p,
    )(pname, data);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetIntegerv({:?}, {:?}): {}", pname, data, err);
      }
    }
    out
  }
  /// See [glGetInternalformativ](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetInternalformativ.xhtml)
  #[inline]
  pub unsafe fn glGetInternalformativ(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    bufSize: GLsizei,
    params: *mut GLint,
  ) {
    trace!("glGetInternalformativ");
    let p: *mut c_void = {
      let temp_p = storage::GetInternalformativ.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetInternalformativ is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint),
    >(p)(target, internalformat, pname, bufSize, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetInternalformativ({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, internalformat, pname, bufSize, params, err
        );
      }
    }
    out
  }
  /// See [glGetProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetProgramBinary.xhtml)
  ///
  /// Fallbacks: GetProgramBinaryOES
  #[inline]
  pub unsafe fn glGetProgramBinary(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    binaryFormat: *mut GLenum,
    binary: *mut c_void,
  ) {
    trace!("glGetProgramBinary");
    let p: *mut c_void = {
      let temp_p = storage::GetProgramBinary.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetProgramBinary is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLuint,
        GLsizei,
        *mut GLsizei,
        *mut GLenum,
        *mut c_void,
      ),
    >(p)(program, bufSize, length, binaryFormat, binary);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetProgramBinary({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          program, bufSize, length, binaryFormat, binary, err
        );
      }
    }
    out
  }
  /// See [glGetProgramInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetProgramInfoLog.xhtml)
  #[inline]
  pub unsafe fn glGetProgramInfoLog(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
  ) {
    trace!("glGetProgramInfoLog");
    let p: *mut c_void = {
      let temp_p = storage::GetProgramInfoLog.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetProgramInfoLog is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    >(p)(program, bufSize, length, infoLog);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetProgramInfoLog({:?}, {:?}, {:?}, {:?}): {}",
          program, bufSize, length, infoLog, err
        );
      }
    }
    out
  }
  /// See [glGetProgramiv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetProgramiv.xhtml)
  #[inline]
  pub unsafe fn glGetProgramiv(
    program: GLuint,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetProgramiv");
    let p: *mut c_void = {
      let temp_p = storage::GetProgramiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetProgramiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLint),
    >(p)(program, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetProgramiv({:?}, {:?}, {:?}): {}",
          program, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetQueryObjectuiv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetQueryObjectuiv.xhtml)
  ///
  /// Fallbacks: GetQueryObjectuivARB
  #[inline]
  pub unsafe fn glGetQueryObjectuiv(
    id: GLuint,
    pname: GLenum,
    params: *mut GLuint,
  ) {
    trace!("glGetQueryObjectuiv");
    let p: *mut c_void = {
      let temp_p = storage::GetQueryObjectuiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetQueryObjectuiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLuint),
    >(p)(id, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetQueryObjectuiv({:?}, {:?}, {:?}): {}",
          id, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetQueryiv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetQueryiv.xhtml)
  ///
  /// Fallbacks: GetQueryivARB
  #[inline]
  pub unsafe fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    trace!("glGetQueryiv");
    let p: *mut c_void = {
      let temp_p = storage::GetQueryiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetQueryiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *mut GLint),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetQueryiv({:?}, {:?}, {:?}): {}", target, pname, params, err);
      }
    }
    out
  }
  /// See [glGetRenderbufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetRenderbufferParameteriv.xhtml)
  ///
  /// Fallbacks: GetRenderbufferParameterivEXT
  #[inline]
  pub unsafe fn glGetRenderbufferParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetRenderbufferParameteriv");
    let p: *mut c_void = {
      let temp_p = storage::GetRenderbufferParameteriv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetRenderbufferParameteriv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *mut GLint),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetRenderbufferParameteriv({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetSamplerParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetSamplerParameter.xhtml)
  #[inline]
  pub unsafe fn glGetSamplerParameterfv(
    sampler: GLuint,
    pname: GLenum,
    params: *mut GLfloat,
  ) {
    trace!("glGetSamplerParameterfv");
    let p: *mut c_void = {
      let temp_p = storage::GetSamplerParameterfv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetSamplerParameterfv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLfloat),
    >(p)(sampler, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetSamplerParameterfv({:?}, {:?}, {:?}): {}",
          sampler, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetSamplerParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetSamplerParameter.xhtml)
  #[inline]
  pub unsafe fn glGetSamplerParameteriv(
    sampler: GLuint,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetSamplerParameteriv");
    let p: *mut c_void = {
      let temp_p = storage::GetSamplerParameteriv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetSamplerParameteriv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLint),
    >(p)(sampler, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetSamplerParameteriv({:?}, {:?}, {:?}): {}",
          sampler, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetShaderInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetShaderInfoLog.xhtml)
  #[inline]
  pub unsafe fn glGetShaderInfoLog(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
  ) {
    trace!("glGetShaderInfoLog");
    let p: *mut c_void = {
      let temp_p = storage::GetShaderInfoLog.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetShaderInfoLog is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    >(p)(shader, bufSize, length, infoLog);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetShaderInfoLog({:?}, {:?}, {:?}, {:?}): {}",
          shader, bufSize, length, infoLog, err
        );
      }
    }
    out
  }
  /// See [glGetShaderPrecisionFormat](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetShaderPrecisionFormat.xhtml)
  #[inline]
  pub unsafe fn glGetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
  ) {
    trace!("glGetShaderPrecisionFormat");
    let p: *mut c_void = {
      let temp_p = storage::GetShaderPrecisionFormat.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetShaderPrecisionFormat is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint),
    >(p)(shadertype, precisiontype, range, precision);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetShaderPrecisionFormat({:?}, {:?}, {:?}, {:?}): {}",
          shadertype, precisiontype, range, precision, err
        );
      }
    }
    out
  }
  /// See [glGetShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetShaderSource.xhtml)
  ///
  /// Fallbacks: GetShaderSourceARB
  #[inline]
  pub unsafe fn glGetShaderSource(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
  ) {
    trace!("glGetShaderSource");
    let p: *mut c_void = {
      let temp_p = storage::GetShaderSource.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetShaderSource is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
    >(p)(shader, bufSize, length, source);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetShaderSource({:?}, {:?}, {:?}, {:?}): {}",
          shader, bufSize, length, source, err
        );
      }
    }
    out
  }
  /// See [glGetShaderiv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetShaderiv.xhtml)
  #[inline]
  pub unsafe fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    trace!("glGetShaderiv");
    let p: *mut c_void = {
      let temp_p = storage::GetShaderiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetShaderiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLint),
    >(p)(shader, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetShaderiv({:?}, {:?}, {:?}): {}", shader, pname, params, err);
      }
    }
    out
  }
  /// See [glGetString](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetString.xhtml)
  #[inline]
  pub unsafe fn glGetString(name: GLenum) -> *const GLubyte {
    trace!("glGetString");
    let p: *mut c_void = {
      let temp_p = storage::GetString.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetString is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum) -> *const GLubyte,
    >(p)(name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetString({:?}): {}", name, err);
      }
    }
    out
  }
  /// See [glGetStringi](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetStringi.xhtml)
  #[inline]
  pub unsafe fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
    trace!("glGetStringi");
    let p: *mut c_void = {
      let temp_p = storage::GetStringi.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetStringi is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLuint) -> *const GLubyte,
    >(p)(name, index);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetStringi({:?}, {:?}): {}", name, index, err);
      }
    }
    out
  }
  /// See [glGetSynciv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetSynciv.xhtml)
  ///
  /// Fallbacks: GetSyncivAPPLE
  #[inline]
  pub unsafe fn glGetSynciv(
    sync: GLsync,
    pname: GLenum,
    bufSize: GLsizei,
    length: *mut GLsizei,
    values: *mut GLint,
  ) {
    trace!("glGetSynciv");
    let p: *mut c_void = {
      let temp_p = storage::GetSynciv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetSynciv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint),
    >(p)(sync, pname, bufSize, length, values);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetSynciv({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          sync, pname, bufSize, length, values, err
        );
      }
    }
    out
  }
  /// See [glGetTexParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetTexParameter.xhtml)
  #[inline]
  pub unsafe fn glGetTexParameterfv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLfloat,
  ) {
    trace!("glGetTexParameterfv");
    let p: *mut c_void = {
      let temp_p = storage::GetTexParameterfv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetTexParameterfv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *mut GLfloat),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetTexParameterfv({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetTexParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetTexParameter.xhtml)
  #[inline]
  pub unsafe fn glGetTexParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetTexParameteriv");
    let p: *mut c_void = {
      let temp_p = storage::GetTexParameteriv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetTexParameteriv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *mut GLint),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetTexParameteriv({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetTransformFeedbackVarying](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetTransformFeedbackVarying.xhtml)
  ///
  /// Fallbacks: GetTransformFeedbackVaryingEXT
  #[inline]
  pub unsafe fn glGetTransformFeedbackVarying(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLsizei,
    type_: *mut GLenum,
    name: *mut GLchar,
  ) {
    trace!("glGetTransformFeedbackVarying");
    let p: *mut c_void = {
      let temp_p = storage::GetTransformFeedbackVarying.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetTransformFeedbackVarying is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLuint,
        GLuint,
        GLsizei,
        *mut GLsizei,
        *mut GLsizei,
        *mut GLenum,
        *mut GLchar,
      ),
    >(p)(program, index, bufSize, length, size, type_, name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetTransformFeedbackVarying({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", program, index, bufSize, length, size, type_, name, err);
      }
    }
    out
  }
  /// See [glGetUniformBlockIndex](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetUniformBlockIndex.xhtml)
  #[inline]
  pub unsafe fn glGetUniformBlockIndex(
    program: GLuint,
    uniformBlockName: *const GLchar,
  ) -> GLuint {
    trace!("glGetUniformBlockIndex");
    let p: *mut c_void = {
      let temp_p = storage::GetUniformBlockIndex.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetUniformBlockIndex is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLchar) -> GLuint,
    >(p)(program, uniformBlockName);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetUniformBlockIndex({:?}, {:?}): {}",
          program, uniformBlockName, err
        );
      }
    }
    out
  }
  /// See [glGetUniformIndices](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetUniformIndices.xhtml)
  #[inline]
  pub unsafe fn glGetUniformIndices(
    program: GLuint,
    uniformCount: GLsizei,
    uniformNames: *const *const GLchar,
    uniformIndices: *mut GLuint,
  ) {
    trace!("glGetUniformIndices");
    let p: *mut c_void = {
      let temp_p = storage::GetUniformIndices.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetUniformIndices is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint),
    >(p)(program, uniformCount, uniformNames, uniformIndices);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetUniformIndices({:?}, {:?}, {:?}, {:?}): {}",
          program, uniformCount, uniformNames, uniformIndices, err
        );
      }
    }
    out
  }
  /// See [glGetUniformLocation](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetUniformLocation.xhtml)
  ///
  /// Fallbacks: GetUniformLocationARB
  #[inline]
  pub unsafe fn glGetUniformLocation(
    program: GLuint,
    name: *const GLchar,
  ) -> GLint {
    trace!("glGetUniformLocation");
    let p: *mut c_void = {
      let temp_p = storage::GetUniformLocation.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetUniformLocation is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLchar) -> GLint,
    >(p)(program, name);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetUniformLocation({:?}, {:?}): {}", program, name, err);
      }
    }
    out
  }
  /// See [glGetUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetUniform.xhtml)
  ///
  /// Fallbacks: GetUniformfvARB
  #[inline]
  pub unsafe fn glGetUniformfv(
    program: GLuint,
    location: GLint,
    params: *mut GLfloat,
  ) {
    trace!("glGetUniformfv");
    let p: *mut c_void = {
      let temp_p = storage::GetUniformfv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetUniformfv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLint, *mut GLfloat),
    >(p)(program, location, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetUniformfv({:?}, {:?}, {:?}): {}",
          program, location, params, err
        );
      }
    }
    out
  }
  /// See [glGetUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetUniform.xhtml)
  ///
  /// Fallbacks: GetUniformivARB
  #[inline]
  pub unsafe fn glGetUniformiv(
    program: GLuint,
    location: GLint,
    params: *mut GLint,
  ) {
    trace!("glGetUniformiv");
    let p: *mut c_void = {
      let temp_p = storage::GetUniformiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetUniformiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLint, *mut GLint),
    >(p)(program, location, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetUniformiv({:?}, {:?}, {:?}): {}",
          program, location, params, err
        );
      }
    }
    out
  }
  /// See [glGetUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetUniform.xhtml)
  ///
  /// Fallbacks: GetUniformuivEXT
  #[inline]
  pub unsafe fn glGetUniformuiv(
    program: GLuint,
    location: GLint,
    params: *mut GLuint,
  ) {
    trace!("glGetUniformuiv");
    let p: *mut c_void = {
      let temp_p = storage::GetUniformuiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetUniformuiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLint, *mut GLuint),
    >(p)(program, location, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetUniformuiv({:?}, {:?}, {:?}): {}",
          program, location, params, err
        );
      }
    }
    out
  }
  /// See [glGetVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetVertexAttrib.xhtml)
  ///
  /// Fallbacks: GetVertexAttribIivEXT
  #[inline]
  pub unsafe fn glGetVertexAttribIiv(
    index: GLuint,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetVertexAttribIiv");
    let p: *mut c_void = {
      let temp_p = storage::GetVertexAttribIiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetVertexAttribIiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLint),
    >(p)(index, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetVertexAttribIiv({:?}, {:?}, {:?}): {}",
          index, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetVertexAttrib.xhtml)
  ///
  /// Fallbacks: GetVertexAttribIuivEXT
  #[inline]
  pub unsafe fn glGetVertexAttribIuiv(
    index: GLuint,
    pname: GLenum,
    params: *mut GLuint,
  ) {
    trace!("glGetVertexAttribIuiv");
    let p: *mut c_void = {
      let temp_p = storage::GetVertexAttribIuiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetVertexAttribIuiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLuint),
    >(p)(index, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetVertexAttribIuiv({:?}, {:?}, {:?}): {}",
          index, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetVertexAttribPointerv](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetVertexAttribPointerv.xhtml)
  ///
  /// Fallbacks: GetVertexAttribPointervARB, GetVertexAttribPointervNV
  #[inline]
  pub unsafe fn glGetVertexAttribPointerv(
    index: GLuint,
    pname: GLenum,
    pointer: *const *mut c_void,
  ) {
    trace!("glGetVertexAttribPointerv");
    let p: *mut c_void = {
      let temp_p = storage::GetVertexAttribPointerv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetVertexAttribPointerv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *const *mut c_void),
    >(p)(index, pname, pointer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetVertexAttribPointerv({:?}, {:?}, {:?}): {}",
          index, pname, pointer, err
        );
      }
    }
    out
  }
  /// See [glGetVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetVertexAttrib.xhtml)
  ///
  /// Fallbacks: GetVertexAttribfvARB, GetVertexAttribfvNV
  #[inline]
  pub unsafe fn glGetVertexAttribfv(
    index: GLuint,
    pname: GLenum,
    params: *mut GLfloat,
  ) {
    trace!("glGetVertexAttribfv");
    let p: *mut c_void = {
      let temp_p = storage::GetVertexAttribfv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetVertexAttribfv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLfloat),
    >(p)(index, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetVertexAttribfv({:?}, {:?}, {:?}): {}",
          index, pname, params, err
        );
      }
    }
    out
  }
  /// See [glGetVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glGetVertexAttrib.xhtml)
  ///
  /// Fallbacks: GetVertexAttribivARB, GetVertexAttribivNV
  #[inline]
  pub unsafe fn glGetVertexAttribiv(
    index: GLuint,
    pname: GLenum,
    params: *mut GLint,
  ) {
    trace!("glGetVertexAttribiv");
    let p: *mut c_void = {
      let temp_p = storage::GetVertexAttribiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glGetVertexAttribiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *mut GLint),
    >(p)(index, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glGetVertexAttribiv({:?}, {:?}, {:?}): {}",
          index, pname, params, err
        );
      }
    }
    out
  }
  /// See [glHint](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glHint.xhtml)
  #[inline]
  pub unsafe fn glHint(target: GLenum, mode: GLenum) {
    trace!("glHint");
    let p: *mut c_void = {
      let temp_p = storage::Hint.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glHint is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLenum)>(p)(
      target, mode,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glHint({:?}, {:?}): {}", target, mode, err);
      }
    }
    out
  }
  /// See [glInvalidateFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glInvalidateFramebuffer.xhtml)
  #[inline]
  pub unsafe fn glInvalidateFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: *const GLenum,
  ) {
    trace!("glInvalidateFramebuffer");
    let p: *mut c_void = {
      let temp_p = storage::InvalidateFramebuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glInvalidateFramebuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLsizei, *const GLenum),
    >(p)(target, numAttachments, attachments);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glInvalidateFramebuffer({:?}, {:?}, {:?}): {}",
          target, numAttachments, attachments, err
        );
      }
    }
    out
  }
  /// See [glInvalidateSubFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glInvalidateSubFramebuffer.xhtml)
  #[inline]
  pub unsafe fn glInvalidateSubFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  ) {
    trace!("glInvalidateSubFramebuffer");
    let p: *mut c_void = {
      let temp_p = storage::InvalidateSubFramebuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glInvalidateSubFramebuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLsizei,
        *const GLenum,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
      ),
    >(p)(target, numAttachments, attachments, x, y, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glInvalidateSubFramebuffer({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, numAttachments, attachments, x, y, width, height, err);
      }
    }
    out
  }
  /// See [glIsBuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsBuffer.xhtml)
  ///
  /// Fallbacks: IsBufferARB
  #[inline]
  pub unsafe fn glIsBuffer(buffer: GLuint) -> GLboolean {
    trace!("glIsBuffer");
    let p: *mut c_void = {
      let temp_p = storage::IsBuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsBuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(buffer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsBuffer({:?}): {}", buffer, err);
      }
    }
    out
  }
  /// See [glIsEnabled](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsEnabled.xhtml)
  #[inline]
  pub unsafe fn glIsEnabled(cap: GLenum) -> GLboolean {
    trace!("glIsEnabled");
    let p: *mut c_void = {
      let temp_p = storage::IsEnabled.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsEnabled is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<*mut c_void, extern "system" fn(GLenum) -> GLboolean>(p)(cap);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsEnabled({:?}): {}", cap, err);
      }
    }
    out
  }
  /// See [glIsFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsFramebuffer.xhtml)
  ///
  /// Fallbacks: IsFramebufferEXT
  #[inline]
  pub unsafe fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
    trace!("glIsFramebuffer");
    let p: *mut c_void = {
      let temp_p = storage::IsFramebuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsFramebuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(framebuffer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsFramebuffer({:?}): {}", framebuffer, err);
      }
    }
    out
  }
  /// See [glIsProgram](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsProgram.xhtml)
  #[inline]
  pub unsafe fn glIsProgram(program: GLuint) -> GLboolean {
    trace!("glIsProgram");
    let p: *mut c_void = {
      let temp_p = storage::IsProgram.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsProgram is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(program);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsProgram({:?}): {}", program, err);
      }
    }
    out
  }
  /// See [glIsQuery](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsQuery.xhtml)
  ///
  /// Fallbacks: IsQueryARB
  #[inline]
  pub unsafe fn glIsQuery(id: GLuint) -> GLboolean {
    trace!("glIsQuery");
    let p: *mut c_void = {
      let temp_p = storage::IsQuery.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsQuery is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(p)(id);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsQuery({:?}): {}", id, err);
      }
    }
    out
  }
  /// See [glIsRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsRenderbuffer.xhtml)
  ///
  /// Fallbacks: IsRenderbufferEXT
  #[inline]
  pub unsafe fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    trace!("glIsRenderbuffer");
    let p: *mut c_void = {
      let temp_p = storage::IsRenderbuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsRenderbuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(renderbuffer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsRenderbuffer({:?}): {}", renderbuffer, err);
      }
    }
    out
  }
  /// See [glIsSampler](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsSampler.xhtml)
  #[inline]
  pub unsafe fn glIsSampler(sampler: GLuint) -> GLboolean {
    trace!("glIsSampler");
    let p: *mut c_void = {
      let temp_p = storage::IsSampler.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsSampler is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(sampler);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsSampler({:?}): {}", sampler, err);
      }
    }
    out
  }
  /// See [glIsShader](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsShader.xhtml)
  #[inline]
  pub unsafe fn glIsShader(shader: GLuint) -> GLboolean {
    trace!("glIsShader");
    let p: *mut c_void = {
      let temp_p = storage::IsShader.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsShader is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(shader);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsShader({:?}): {}", shader, err);
      }
    }
    out
  }
  /// See [glIsSync](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsSync.xhtml)
  ///
  /// Fallbacks: IsSyncAPPLE
  #[inline]
  pub unsafe fn glIsSync(sync: GLsync) -> GLboolean {
    trace!("glIsSync");
    let p: *mut c_void = {
      let temp_p = storage::IsSync.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsSync is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLsync) -> GLboolean>(
      p,
    )(sync);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsSync({:?}): {}", sync, err);
      }
    }
    out
  }
  /// See [glIsTexture](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsTexture.xhtml)
  #[inline]
  pub unsafe fn glIsTexture(texture: GLuint) -> GLboolean {
    trace!("glIsTexture");
    let p: *mut c_void = {
      let temp_p = storage::IsTexture.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsTexture is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(texture);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsTexture({:?}): {}", texture, err);
      }
    }
    out
  }
  /// See [glIsTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsTransformFeedback.xhtml)
  ///
  /// Fallbacks: IsTransformFeedbackNV
  #[inline]
  pub unsafe fn glIsTransformFeedback(id: GLuint) -> GLboolean {
    trace!("glIsTransformFeedback");
    let p: *mut c_void = {
      let temp_p = storage::IsTransformFeedback.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsTransformFeedback is not loaded");
      }
      temp_p
    };
    let out =
      transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(p)(id);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsTransformFeedback({:?}): {}", id, err);
      }
    }
    out
  }
  /// See [glIsVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glIsVertexArray.xhtml)
  ///
  /// Fallbacks: IsVertexArrayAPPLE, IsVertexArrayOES
  #[inline]
  pub unsafe fn glIsVertexArray(array: GLuint) -> GLboolean {
    trace!("glIsVertexArray");
    let p: *mut c_void = {
      let temp_p = storage::IsVertexArray.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glIsVertexArray is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint) -> GLboolean>(
      p,
    )(array);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glIsVertexArray({:?}): {}", array, err);
      }
    }
    out
  }
  /// See [glLineWidth](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glLineWidth.xhtml)
  #[inline]
  pub unsafe fn glLineWidth(width: GLfloat) {
    trace!("glLineWidth");
    let p: *mut c_void = {
      let temp_p = storage::LineWidth.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glLineWidth is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLfloat)>(p)(width);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glLineWidth({:?}): {}", width, err);
      }
    }
    out
  }
  /// See [glLinkProgram](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glLinkProgram.xhtml)
  ///
  /// Fallbacks: LinkProgramARB
  #[inline]
  pub unsafe fn glLinkProgram(program: GLuint) {
    trace!("glLinkProgram");
    let p: *mut c_void = {
      let temp_p = storage::LinkProgram.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glLinkProgram is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(program);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glLinkProgram({:?}): {}", program, err);
      }
    }
    out
  }
  /// See [glMapBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glMapBufferRange.xhtml)
  ///
  /// Fallbacks: MapBufferRangeEXT
  #[inline]
  pub unsafe fn glMapBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
  ) -> *mut c_void {
    trace!("glMapBufferRange");
    let p: *mut c_void = {
      let temp_p = storage::MapBufferRange.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glMapBufferRange is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLintptr,
        GLsizeiptr,
        GLbitfield,
      ) -> *mut c_void,
    >(p)(target, offset, length, access);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glMapBufferRange({:?}, {:?}, {:?}, {:?}): {}",
          target, offset, length, access, err
        );
      }
    }
    out
  }
  /// See [glPauseTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glPauseTransformFeedback.xhtml)
  ///
  /// Fallbacks: PauseTransformFeedbackNV
  #[inline]
  pub unsafe fn glPauseTransformFeedback() {
    trace!("glPauseTransformFeedback");
    let p: *mut c_void = {
      let temp_p = storage::PauseTransformFeedback.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glPauseTransformFeedback is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn()>(p)();
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glPauseTransformFeedback(): {}", err);
      }
    }
    out
  }
  /// See [glPixelStorei](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glPixelStorei.xhtml)
  #[inline]
  pub unsafe fn glPixelStorei(pname: GLenum, param: GLint) {
    trace!("glPixelStorei");
    let p: *mut c_void = {
      let temp_p = storage::PixelStorei.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glPixelStorei is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLint)>(p)(
      pname, param,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glPixelStorei({:?}, {:?}): {}", pname, param, err);
      }
    }
    out
  }
  /// See [glPolygonOffset](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glPolygonOffset.xhtml)
  #[inline]
  pub unsafe fn glPolygonOffset(factor: GLfloat, units: GLfloat) {
    trace!("glPolygonOffset");
    let p: *mut c_void = {
      let temp_p = storage::PolygonOffset.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glPolygonOffset is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLfloat, GLfloat)>(p)(
      factor, units,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glPolygonOffset({:?}, {:?}): {}", factor, units, err);
      }
    }
    out
  }
  /// See [glProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glProgramBinary.xhtml)
  ///
  /// Fallbacks: ProgramBinaryOES
  #[inline]
  pub unsafe fn glProgramBinary(
    program: GLuint,
    binaryFormat: GLenum,
    binary: *const c_void,
    length: GLsizei,
  ) {
    trace!("glProgramBinary");
    let p: *mut c_void = {
      let temp_p = storage::ProgramBinary.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glProgramBinary is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *const c_void, GLsizei),
    >(p)(program, binaryFormat, binary, length);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glProgramBinary({:?}, {:?}, {:?}, {:?}): {}",
          program, binaryFormat, binary, length, err
        );
      }
    }
    out
  }
  /// See [glProgramParameteri](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glProgramParameteri.xhtml)
  ///
  /// Fallbacks: ProgramParameteriARB, ProgramParameteriEXT
  #[inline]
  pub unsafe fn glProgramParameteri(
    program: GLuint,
    pname: GLenum,
    value: GLint,
  ) {
    trace!("glProgramParameteri");
    let p: *mut c_void = {
      let temp_p = storage::ProgramParameteri.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glProgramParameteri is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, GLenum, GLint)>(
      p,
    )(program, pname, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glProgramParameteri({:?}, {:?}, {:?}): {}",
          program, pname, value, err
        );
      }
    }
    out
  }
  /// See [glReadBuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glReadBuffer.xhtml)
  #[inline]
  pub unsafe fn glReadBuffer(src: GLenum) {
    trace!("glReadBuffer");
    let p: *mut c_void = {
      let temp_p = storage::ReadBuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glReadBuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum)>(p)(src);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glReadBuffer({:?}): {}", src, err);
      }
    }
    out
  }
  /// See [glReadPixels](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glReadPixels.xhtml)
  #[inline]
  pub unsafe fn glReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
  ) {
    trace!("glReadPixels");
    let p: *mut c_void = {
      let temp_p = storage::ReadPixels.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glReadPixels is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *mut c_void,
      ),
    >(p)(x, y, width, height, format, type_, pixels);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glReadPixels({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}",
          x, y, width, height, format, type_, pixels, err
        );
      }
    }
    out
  }
  /// See [glReleaseShaderCompiler](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glReleaseShaderCompiler.xhtml)
  #[inline]
  pub unsafe fn glReleaseShaderCompiler() {
    trace!("glReleaseShaderCompiler");
    let p: *mut c_void = {
      let temp_p = storage::ReleaseShaderCompiler.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glReleaseShaderCompiler is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn()>(p)();
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glReleaseShaderCompiler(): {}", err);
      }
    }
    out
  }
  /// See [glRenderbufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glRenderbufferStorage.xhtml)
  ///
  /// Fallbacks: RenderbufferStorageEXT
  #[inline]
  pub unsafe fn glRenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  ) {
    trace!("glRenderbufferStorage");
    let p: *mut c_void = {
      let temp_p = storage::RenderbufferStorage.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glRenderbufferStorage is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLsizei, GLsizei),
    >(p)(target, internalformat, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glRenderbufferStorage({:?}, {:?}, {:?}, {:?}): {}",
          target, internalformat, width, height, err
        );
      }
    }
    out
  }
  /// See [glRenderbufferStorageMultisample](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glRenderbufferStorageMultisample.xhtml)
  ///
  /// Fallbacks: RenderbufferStorageMultisampleEXT,
  /// RenderbufferStorageMultisampleNV
  #[inline]
  pub unsafe fn glRenderbufferStorageMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  ) {
    trace!("glRenderbufferStorageMultisample");
    let p: *mut c_void = {
      let temp_p =
        storage::RenderbufferStorageMultisample.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glRenderbufferStorageMultisample is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei),
    >(p)(target, samples, internalformat, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glRenderbufferStorageMultisample({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, samples, internalformat, width, height, err
        );
      }
    }
    out
  }
  /// See [glResumeTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glResumeTransformFeedback.xhtml)
  ///
  /// Fallbacks: ResumeTransformFeedbackNV
  #[inline]
  pub unsafe fn glResumeTransformFeedback() {
    trace!("glResumeTransformFeedback");
    let p: *mut c_void = {
      let temp_p = storage::ResumeTransformFeedback.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glResumeTransformFeedback is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn()>(p)();
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glResumeTransformFeedback(): {}", err);
      }
    }
    out
  }
  /// See [glSampleCoverage](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glSampleCoverage.xhtml)
  ///
  /// Fallbacks: SampleCoverageARB
  #[inline]
  pub unsafe fn glSampleCoverage(value: GLfloat, invert: GLboolean) {
    trace!("glSampleCoverage");
    let p: *mut c_void = {
      let temp_p = storage::SampleCoverage.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glSampleCoverage is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLfloat, GLboolean)>(
      p,
    )(value, invert);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glSampleCoverage({:?}, {:?}): {}", value, invert, err);
      }
    }
    out
  }
  /// See [glSamplerParameterf](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glSamplerParameterf.xhtml)
  #[inline]
  pub unsafe fn glSamplerParameterf(
    sampler: GLuint,
    pname: GLenum,
    param: GLfloat,
  ) {
    trace!("glSamplerParameterf");
    let p: *mut c_void = {
      let temp_p = storage::SamplerParameterf.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glSamplerParameterf is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, GLfloat),
    >(p)(sampler, pname, param);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glSamplerParameterf({:?}, {:?}, {:?}): {}",
          sampler, pname, param, err
        );
      }
    }
    out
  }
  /// See [glSamplerParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glSamplerParameter.xhtml)
  #[inline]
  pub unsafe fn glSamplerParameterfv(
    sampler: GLuint,
    pname: GLenum,
    param: *const GLfloat,
  ) {
    trace!("glSamplerParameterfv");
    let p: *mut c_void = {
      let temp_p = storage::SamplerParameterfv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glSamplerParameterfv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *const GLfloat),
    >(p)(sampler, pname, param);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glSamplerParameterfv({:?}, {:?}, {:?}): {}",
          sampler, pname, param, err
        );
      }
    }
    out
  }
  /// See [glSamplerParameteri](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glSamplerParameteri.xhtml)
  #[inline]
  pub unsafe fn glSamplerParameteri(
    sampler: GLuint,
    pname: GLenum,
    param: GLint,
  ) {
    trace!("glSamplerParameteri");
    let p: *mut c_void = {
      let temp_p = storage::SamplerParameteri.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glSamplerParameteri is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, GLenum, GLint)>(
      p,
    )(sampler, pname, param);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glSamplerParameteri({:?}, {:?}, {:?}): {}",
          sampler, pname, param, err
        );
      }
    }
    out
  }
  /// See [glSamplerParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glSamplerParameter.xhtml)
  #[inline]
  pub unsafe fn glSamplerParameteriv(
    sampler: GLuint,
    pname: GLenum,
    param: *const GLint,
  ) {
    trace!("glSamplerParameteriv");
    let p: *mut c_void = {
      let temp_p = storage::SamplerParameteriv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glSamplerParameteriv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLenum, *const GLint),
    >(p)(sampler, pname, param);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glSamplerParameteriv({:?}, {:?}, {:?}): {}",
          sampler, pname, param, err
        );
      }
    }
    out
  }
  /// See [glScissor](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glScissor.xhtml)
  #[inline]
  pub unsafe fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    trace!("glScissor");
    let p: *mut c_void = {
      let temp_p = storage::Scissor.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glScissor is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLint, GLsizei, GLsizei),
    >(p)(x, y, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glScissor({:?}, {:?}, {:?}, {:?}): {}", x, y, width, height, err);
      }
    }
    out
  }
  /// See [glShaderBinary](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glShaderBinary.xhtml)
  #[inline]
  pub unsafe fn glShaderBinary(
    count: GLsizei,
    shaders: *const GLuint,
    binaryformat: GLenum,
    binary: *const c_void,
    length: GLsizei,
  ) {
    trace!("glShaderBinary");
    let p: *mut c_void = {
      let temp_p = storage::ShaderBinary.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glShaderBinary is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLsizei,
        *const GLuint,
        GLenum,
        *const c_void,
        GLsizei,
      ),
    >(p)(count, shaders, binaryformat, binary, length);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glShaderBinary({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          count, shaders, binaryformat, binary, length, err
        );
      }
    }
    out
  }
  /// See [glShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glShaderSource.xhtml)
  ///
  /// Fallbacks: ShaderSourceARB
  #[inline]
  pub unsafe fn glShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
  ) {
    trace!("glShaderSource");
    let p: *mut c_void = {
      let temp_p = storage::ShaderSource.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glShaderSource is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint),
    >(p)(shader, count, string, length);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glShaderSource({:?}, {:?}, {:?}, {:?}): {}",
          shader, count, string, length, err
        );
      }
    }
    out
  }
  /// See [glStencilFunc](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glStencilFunc.xhtml)
  #[inline]
  pub unsafe fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint) {
    trace!("glStencilFunc");
    let p: *mut c_void = {
      let temp_p = storage::StencilFunc.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glStencilFunc is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLint, GLuint)>(
      p,
    )(func, ref_, mask);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glStencilFunc({:?}, {:?}, {:?}): {}", func, ref_, mask, err);
      }
    }
    out
  }
  /// See [glStencilFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glStencilFuncSeparate.xhtml)
  #[inline]
  pub unsafe fn glStencilFuncSeparate(
    face: GLenum,
    func: GLenum,
    ref_: GLint,
    mask: GLuint,
  ) {
    trace!("glStencilFuncSeparate");
    let p: *mut c_void = {
      let temp_p = storage::StencilFuncSeparate.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glStencilFuncSeparate is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLint, GLuint),
    >(p)(face, func, ref_, mask);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glStencilFuncSeparate({:?}, {:?}, {:?}, {:?}): {}",
          face, func, ref_, mask, err
        );
      }
    }
    out
  }
  /// See [glStencilMask](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glStencilMask.xhtml)
  #[inline]
  pub unsafe fn glStencilMask(mask: GLuint) {
    trace!("glStencilMask");
    let p: *mut c_void = {
      let temp_p = storage::StencilMask.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glStencilMask is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(mask);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glStencilMask({:?}): {}", mask, err);
      }
    }
    out
  }
  /// See [glStencilMaskSeparate](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glStencilMaskSeparate.xhtml)
  #[inline]
  pub unsafe fn glStencilMaskSeparate(face: GLenum, mask: GLuint) {
    trace!("glStencilMaskSeparate");
    let p: *mut c_void = {
      let temp_p = storage::StencilMaskSeparate.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glStencilMaskSeparate is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLuint)>(p)(
      face, mask,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glStencilMaskSeparate({:?}, {:?}): {}", face, mask, err);
      }
    }
    out
  }
  /// See [glStencilOp](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glStencilOp.xhtml)
  #[inline]
  pub unsafe fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    trace!("glStencilOp");
    let p: *mut c_void = {
      let temp_p = storage::StencilOp.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glStencilOp is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLenum),
    >(p)(fail, zfail, zpass);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glStencilOp({:?}, {:?}, {:?}): {}", fail, zfail, zpass, err);
      }
    }
    out
  }
  /// See [glStencilOpSeparate](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glStencilOpSeparate.xhtml)
  ///
  /// Fallbacks: StencilOpSeparateATI
  #[inline]
  pub unsafe fn glStencilOpSeparate(
    face: GLenum,
    sfail: GLenum,
    dpfail: GLenum,
    dppass: GLenum,
  ) {
    trace!("glStencilOpSeparate");
    let p: *mut c_void = {
      let temp_p = storage::StencilOpSeparate.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glStencilOpSeparate is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLenum, GLenum),
    >(p)(face, sfail, dpfail, dppass);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glStencilOpSeparate({:?}, {:?}, {:?}, {:?}): {}",
          face, sfail, dpfail, dppass, err
        );
      }
    }
    out
  }
  /// See [glTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml)
  #[inline]
  pub unsafe fn glTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  ) {
    trace!("glTexImage2D");
    let p: *mut c_void = {
      let temp_p = storage::TexImage2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexImage2D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLint,
        GLenum,
        GLenum,
        *const c_void,
      ),
    >(p)(
      target,
      level,
      internalformat,
      width,
      height,
      border,
      format,
      type_,
      pixels,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexImage2D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, internalformat, width, height, border, format, type_, pixels, err);
      }
    }
    out
  }
  /// See [glTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage3D.xhtml)
  ///
  /// Fallbacks: TexImage3DEXT
  #[inline]
  pub unsafe fn glTexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  ) {
    trace!("glTexImage3D");
    let p: *mut c_void = {
      let temp_p = storage::TexImage3D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexImage3D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLint,
        GLenum,
        GLenum,
        *const c_void,
      ),
    >(p)(
      target,
      level,
      internalformat,
      width,
      height,
      depth,
      border,
      format,
      type_,
      pixels,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexImage3D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, internalformat, width, height, depth, border, format, type_, pixels, err);
      }
    }
    out
  }
  /// See [glTexParameterf](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexParameterf.xhtml)
  #[inline]
  pub unsafe fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    trace!("glTexParameterf");
    let p: *mut c_void = {
      let temp_p = storage::TexParameterf.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexParameterf is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, GLfloat),
    >(p)(target, pname, param);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexParameterf({:?}, {:?}, {:?}): {}",
          target, pname, param, err
        );
      }
    }
    out
  }
  /// See [glTexParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexParameter.xhtml)
  #[inline]
  pub unsafe fn glTexParameterfv(
    target: GLenum,
    pname: GLenum,
    params: *const GLfloat,
  ) {
    trace!("glTexParameterfv");
    let p: *mut c_void = {
      let temp_p = storage::TexParameterfv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexParameterfv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *const GLfloat),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexParameterfv({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glTexParameteri](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexParameteri.xhtml)
  #[inline]
  pub unsafe fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    trace!("glTexParameteri");
    let p: *mut c_void = {
      let temp_p = storage::TexParameteri.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexParameteri is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum, GLenum, GLint)>(
      p,
    )(target, pname, param);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexParameteri({:?}, {:?}, {:?}): {}",
          target, pname, param, err
        );
      }
    }
    out
  }
  /// See [glTexParameter](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexParameter.xhtml)
  #[inline]
  pub unsafe fn glTexParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *const GLint,
  ) {
    trace!("glTexParameteriv");
    let p: *mut c_void = {
      let temp_p = storage::TexParameteriv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexParameteriv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLenum, *const GLint),
    >(p)(target, pname, params);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexParameteriv({:?}, {:?}, {:?}): {}",
          target, pname, params, err
        );
      }
    }
    out
  }
  /// See [glTexStorage2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexStorage2D.xhtml)
  ///
  /// Fallbacks: TexStorage2DEXT
  #[inline]
  pub unsafe fn glTexStorage2D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  ) {
    trace!("glTexStorage2D");
    let p: *mut c_void = {
      let temp_p = storage::TexStorage2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexStorage2D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei),
    >(p)(target, levels, internalformat, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexStorage2D({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, levels, internalformat, width, height, err
        );
      }
    }
    out
  }
  /// See [glTexStorage3D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexStorage3D.xhtml)
  ///
  /// Fallbacks: TexStorage3DEXT
  #[inline]
  pub unsafe fn glTexStorage3D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
  ) {
    trace!("glTexStorage3D");
    let p: *mut c_void = {
      let temp_p = storage::TexStorage3D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexStorage3D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei),
    >(p)(target, levels, internalformat, width, height, depth);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexStorage3D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}",
          target, levels, internalformat, width, height, depth, err
        );
      }
    }
    out
  }
  /// See [glTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexSubImage2D.xhtml)
  ///
  /// Fallbacks: TexSubImage2DEXT
  #[inline]
  pub unsafe fn glTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  ) {
    trace!("glTexSubImage2D");
    let p: *mut c_void = {
      let temp_p = storage::TexSubImage2D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexSubImage2D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *const c_void,
      ),
    >(p)(
      target, level, xoffset, yoffset, width, height, format, type_, pixels,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexSubImage2D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, xoffset, yoffset, width, height, format, type_, pixels, err);
      }
    }
    out
  }
  /// See [glTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexSubImage3D.xhtml)
  ///
  /// Fallbacks: TexSubImage3DEXT
  #[inline]
  pub unsafe fn glTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  ) {
    trace!("glTexSubImage3D");
    let p: *mut c_void = {
      let temp_p = storage::TexSubImage3D.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTexSubImage3D is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLenum,
        GLint,
        GLint,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLsizei,
        GLenum,
        GLenum,
        *const c_void,
      ),
    >(p)(
      target, level, xoffset, yoffset, zoffset, width, height, depth, format,
      type_, pixels,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTexSubImage3D({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}", target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels, err);
      }
    }
    out
  }
  /// See [glTransformFeedbackVaryings](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTransformFeedbackVaryings.xhtml)
  ///
  /// Fallbacks: TransformFeedbackVaryingsEXT
  #[inline]
  pub unsafe fn glTransformFeedbackVaryings(
    program: GLuint,
    count: GLsizei,
    varyings: *const *const GLchar,
    bufferMode: GLenum,
  ) {
    trace!("glTransformFeedbackVaryings");
    let p: *mut c_void = {
      let temp_p = storage::TransformFeedbackVaryings.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glTransformFeedbackVaryings is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum),
    >(p)(program, count, varyings, bufferMode);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glTransformFeedbackVaryings({:?}, {:?}, {:?}, {:?}): {}",
          program, count, varyings, bufferMode, err
        );
      }
    }
    out
  }
  /// See [glUniform1f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform1f.xhtml)
  ///
  /// Fallbacks: Uniform1fARB
  #[inline]
  pub unsafe fn glUniform1f(location: GLint, v0: GLfloat) {
    trace!("glUniform1f");
    let p: *mut c_void = {
      let temp_p = storage::Uniform1f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform1f is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLint, GLfloat)>(p)(
      location, v0,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform1f({:?}, {:?}): {}", location, v0, err);
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform1fvARB
  #[inline]
  pub unsafe fn glUniform1fv(
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  ) {
    trace!("glUniform1fv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform1fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform1fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLfloat),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform1fv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform1i](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform1i.xhtml)
  ///
  /// Fallbacks: Uniform1iARB
  #[inline]
  pub unsafe fn glUniform1i(location: GLint, v0: GLint) {
    trace!("glUniform1i");
    let p: *mut c_void = {
      let temp_p = storage::Uniform1i.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform1i is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLint, GLint)>(p)(
      location, v0,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform1i({:?}, {:?}): {}", location, v0, err);
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform1ivARB
  #[inline]
  pub unsafe fn glUniform1iv(
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  ) {
    trace!("glUniform1iv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform1iv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform1iv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform1iv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform1ui](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform1ui.xhtml)
  ///
  /// Fallbacks: Uniform1uiEXT
  #[inline]
  pub unsafe fn glUniform1ui(location: GLint, v0: GLuint) {
    trace!("glUniform1ui");
    let p: *mut c_void = {
      let temp_p = storage::Uniform1ui.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform1ui is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLint, GLuint)>(p)(
      location, v0,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform1ui({:?}, {:?}): {}", location, v0, err);
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform1uivEXT
  #[inline]
  pub unsafe fn glUniform1uiv(
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  ) {
    trace!("glUniform1uiv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform1uiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform1uiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLuint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform1uiv({:?}, {:?}, {:?}): {}",
          location, count, value, err
        );
      }
    }
    out
  }
  /// See [glUniform2f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform2f.xhtml)
  ///
  /// Fallbacks: Uniform2fARB
  #[inline]
  pub unsafe fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    trace!("glUniform2f");
    let p: *mut c_void = {
      let temp_p = storage::Uniform2f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform2f is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLfloat, GLfloat),
    >(p)(location, v0, v1);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform2f({:?}, {:?}, {:?}): {}", location, v0, v1, err);
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform2fvARB
  #[inline]
  pub unsafe fn glUniform2fv(
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  ) {
    trace!("glUniform2fv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform2fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform2fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLfloat),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform2fv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform2i](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform2i.xhtml)
  ///
  /// Fallbacks: Uniform2iARB
  #[inline]
  pub unsafe fn glUniform2i(location: GLint, v0: GLint, v1: GLint) {
    trace!("glUniform2i");
    let p: *mut c_void = {
      let temp_p = storage::Uniform2i.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform2i is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLint, GLint, GLint)>(
      p,
    )(location, v0, v1);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform2i({:?}, {:?}, {:?}): {}", location, v0, v1, err);
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform2ivARB
  #[inline]
  pub unsafe fn glUniform2iv(
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  ) {
    trace!("glUniform2iv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform2iv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform2iv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform2iv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform2ui](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform2ui.xhtml)
  ///
  /// Fallbacks: Uniform2uiEXT
  #[inline]
  pub unsafe fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
    trace!("glUniform2ui");
    let p: *mut c_void = {
      let temp_p = storage::Uniform2ui.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform2ui is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLint, GLuint, GLuint)>(
      p,
    )(location, v0, v1);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform2ui({:?}, {:?}, {:?}): {}", location, v0, v1, err);
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform2uivEXT
  #[inline]
  pub unsafe fn glUniform2uiv(
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  ) {
    trace!("glUniform2uiv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform2uiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform2uiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLuint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform2uiv({:?}, {:?}, {:?}): {}",
          location, count, value, err
        );
      }
    }
    out
  }
  /// See [glUniform3f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform3f.xhtml)
  ///
  /// Fallbacks: Uniform3fARB
  #[inline]
  pub unsafe fn glUniform3f(
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
  ) {
    trace!("glUniform3f");
    let p: *mut c_void = {
      let temp_p = storage::Uniform3f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform3f is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLfloat, GLfloat, GLfloat),
    >(p)(location, v0, v1, v2);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform3f({:?}, {:?}, {:?}, {:?}): {}",
          location, v0, v1, v2, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform3fvARB
  #[inline]
  pub unsafe fn glUniform3fv(
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  ) {
    trace!("glUniform3fv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform3fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform3fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLfloat),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform3fv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform3i](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform3i.xhtml)
  ///
  /// Fallbacks: Uniform3iARB
  #[inline]
  pub unsafe fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    trace!("glUniform3i");
    let p: *mut c_void = {
      let temp_p = storage::Uniform3i.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform3i is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLint, GLint, GLint),
    >(p)(location, v0, v1, v2);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform3i({:?}, {:?}, {:?}, {:?}): {}",
          location, v0, v1, v2, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform3ivARB
  #[inline]
  pub unsafe fn glUniform3iv(
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  ) {
    trace!("glUniform3iv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform3iv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform3iv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform3iv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform3ui](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform3ui.xhtml)
  ///
  /// Fallbacks: Uniform3uiEXT
  #[inline]
  pub unsafe fn glUniform3ui(
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
  ) {
    trace!("glUniform3ui");
    let p: *mut c_void = {
      let temp_p = storage::Uniform3ui.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform3ui is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLuint, GLuint, GLuint),
    >(p)(location, v0, v1, v2);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform3ui({:?}, {:?}, {:?}, {:?}): {}",
          location, v0, v1, v2, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform3uivEXT
  #[inline]
  pub unsafe fn glUniform3uiv(
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  ) {
    trace!("glUniform3uiv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform3uiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform3uiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLuint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform3uiv({:?}, {:?}, {:?}): {}",
          location, count, value, err
        );
      }
    }
    out
  }
  /// See [glUniform4f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform4f.xhtml)
  ///
  /// Fallbacks: Uniform4fARB
  #[inline]
  pub unsafe fn glUniform4f(
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
  ) {
    trace!("glUniform4f");
    let p: *mut c_void = {
      let temp_p = storage::Uniform4f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform4f is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat),
    >(p)(location, v0, v1, v2, v3);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform4f({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          location, v0, v1, v2, v3, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform4fvARB
  #[inline]
  pub unsafe fn glUniform4fv(
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
  ) {
    trace!("glUniform4fv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform4fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform4fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLfloat),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform4fv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform4i](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform4i.xhtml)
  ///
  /// Fallbacks: Uniform4iARB
  #[inline]
  pub unsafe fn glUniform4i(
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
    v3: GLint,
  ) {
    trace!("glUniform4i");
    let p: *mut c_void = {
      let temp_p = storage::Uniform4i.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform4i is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLint, GLint, GLint, GLint),
    >(p)(location, v0, v1, v2, v3);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform4i({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          location, v0, v1, v2, v3, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform4ivARB
  #[inline]
  pub unsafe fn glUniform4iv(
    location: GLint,
    count: GLsizei,
    value: *const GLint,
  ) {
    trace!("glUniform4iv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform4iv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform4iv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform4iv({:?}, {:?}, {:?}): {}", location, count, value, err);
      }
    }
    out
  }
  /// See [glUniform4ui](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform4ui.xhtml)
  ///
  /// Fallbacks: Uniform4uiEXT
  #[inline]
  pub unsafe fn glUniform4ui(
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
    v3: GLuint,
  ) {
    trace!("glUniform4ui");
    let p: *mut c_void = {
      let temp_p = storage::Uniform4ui.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform4ui is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint),
    >(p)(location, v0, v1, v2, v3);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform4ui({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          location, v0, v1, v2, v3, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: Uniform4uivEXT
  #[inline]
  pub unsafe fn glUniform4uiv(
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
  ) {
    trace!("glUniform4uiv");
    let p: *mut c_void = {
      let temp_p = storage::Uniform4uiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniform4uiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, *const GLuint),
    >(p)(location, count, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniform4uiv({:?}, {:?}, {:?}): {}",
          location, count, value, err
        );
      }
    }
    out
  }
  /// See [glUniformBlockBinding](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniformBlockBinding.xhtml)
  #[inline]
  pub unsafe fn glUniformBlockBinding(
    program: GLuint,
    uniformBlockIndex: GLuint,
    uniformBlockBinding: GLuint,
  ) {
    trace!("glUniformBlockBinding");
    let p: *mut c_void = {
      let temp_p = storage::UniformBlockBinding.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformBlockBinding is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLuint, GLuint),
    >(p)(program, uniformBlockIndex, uniformBlockBinding);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformBlockBinding({:?}, {:?}, {:?}): {}",
          program, uniformBlockIndex, uniformBlockBinding, err
        );
      }
    }
    out
  }
  /// See [glUniformMatrix](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniformMatrix.xhtml)
  ///
  /// Fallbacks: UniformMatrix2fvARB
  #[inline]
  pub unsafe fn glUniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix2fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix2fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix2fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix2fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix2x3fvNV
  #[inline]
  pub unsafe fn glUniformMatrix2x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix2x3fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix2x3fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix2x3fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix2x3fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix2x4fvNV
  #[inline]
  pub unsafe fn glUniformMatrix2x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix2x4fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix2x4fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix2x4fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix2x4fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix3fvARB
  #[inline]
  pub unsafe fn glUniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix3fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix3fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix3fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix3fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix3x2fvNV
  #[inline]
  pub unsafe fn glUniformMatrix3x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix3x2fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix3x2fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix3x2fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix3x2fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix3x4fvNV
  #[inline]
  pub unsafe fn glUniformMatrix3x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix3x4fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix3x4fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix3x4fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix3x4fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix4fvARB
  #[inline]
  pub unsafe fn glUniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix4fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix4fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix4fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix4fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix4x2fvNV
  #[inline]
  pub unsafe fn glUniformMatrix4x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix4x2fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix4x2fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix4x2fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix4x2fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUniform](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUniform.xhtml)
  ///
  /// Fallbacks: UniformMatrix4x3fvNV
  #[inline]
  pub unsafe fn glUniformMatrix4x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  ) {
    trace!("glUniformMatrix4x3fv");
    let p: *mut c_void = {
      let temp_p = storage::UniformMatrix4x3fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUniformMatrix4x3fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat),
    >(p)(location, count, transpose, value);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUniformMatrix4x3fv({:?}, {:?}, {:?}, {:?}): {}",
          location, count, transpose, value, err
        );
      }
    }
    out
  }
  /// See [glUnmapBuffer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUnmapBuffer.xhtml)
  ///
  /// Fallbacks: UnmapBufferARB, UnmapBufferOES
  #[inline]
  pub unsafe fn glUnmapBuffer(target: GLenum) -> GLboolean {
    trace!("glUnmapBuffer");
    let p: *mut c_void = {
      let temp_p = storage::UnmapBuffer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUnmapBuffer is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLenum) -> GLboolean>(
      p,
    )(target);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUnmapBuffer({:?}): {}", target, err);
      }
    }
    out
  }
  /// See [glUseProgram](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glUseProgram.xhtml)
  ///
  /// Fallbacks: UseProgramObjectARB
  #[inline]
  pub unsafe fn glUseProgram(program: GLuint) {
    trace!("glUseProgram");
    let p: *mut c_void = {
      let temp_p = storage::UseProgram.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glUseProgram is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(program);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glUseProgram({:?}): {}", program, err);
      }
    }
    out
  }
  /// See [glValidateProgram](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glValidateProgram.xhtml)
  ///
  /// Fallbacks: ValidateProgramARB
  #[inline]
  pub unsafe fn glValidateProgram(program: GLuint) {
    trace!("glValidateProgram");
    let p: *mut c_void = {
      let temp_p = storage::ValidateProgram.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glValidateProgram is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint)>(p)(program);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glValidateProgram({:?}): {}", program, err);
      }
    }
    out
  }
  /// See [glVertexAttrib1f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib1f.xhtml)
  ///
  /// Fallbacks: VertexAttrib1fARB, VertexAttrib1fNV
  #[inline]
  pub unsafe fn glVertexAttrib1f(index: GLuint, x: GLfloat) {
    trace!("glVertexAttrib1f");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib1f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib1f is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, GLfloat)>(p)(
      index, x,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib1f({:?}, {:?}): {}", index, x, err);
      }
    }
    out
  }
  /// See [glVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib.xhtml)
  ///
  /// Fallbacks: VertexAttrib1fvARB, VertexAttrib1fvNV
  #[inline]
  pub unsafe fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat) {
    trace!("glVertexAttrib1fv");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib1fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib1fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLfloat),
    >(p)(index, v);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib1fv({:?}, {:?}): {}", index, v, err);
      }
    }
    out
  }
  /// See [glVertexAttrib2f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib2f.xhtml)
  ///
  /// Fallbacks: VertexAttrib2fARB, VertexAttrib2fNV
  #[inline]
  pub unsafe fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    trace!("glVertexAttrib2f");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib2f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib2f is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLfloat, GLfloat),
    >(p)(index, x, y);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib2f({:?}, {:?}, {:?}): {}", index, x, y, err);
      }
    }
    out
  }
  /// See [glVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib.xhtml)
  ///
  /// Fallbacks: VertexAttrib2fvARB, VertexAttrib2fvNV
  #[inline]
  pub unsafe fn glVertexAttrib2fv(index: GLuint, v: *const GLfloat) {
    trace!("glVertexAttrib2fv");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib2fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib2fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLfloat),
    >(p)(index, v);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib2fv({:?}, {:?}): {}", index, v, err);
      }
    }
    out
  }
  /// See [glVertexAttrib3f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib3f.xhtml)
  ///
  /// Fallbacks: VertexAttrib3fARB, VertexAttrib3fNV
  #[inline]
  pub unsafe fn glVertexAttrib3f(
    index: GLuint,
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
  ) {
    trace!("glVertexAttrib3f");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib3f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib3f is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat),
    >(p)(index, x, y, z);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib3f({:?}, {:?}, {:?}, {:?}): {}",
          index, x, y, z, err
        );
      }
    }
    out
  }
  /// See [glVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib.xhtml)
  ///
  /// Fallbacks: VertexAttrib3fvARB, VertexAttrib3fvNV
  #[inline]
  pub unsafe fn glVertexAttrib3fv(index: GLuint, v: *const GLfloat) {
    trace!("glVertexAttrib3fv");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib3fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib3fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLfloat),
    >(p)(index, v);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib3fv({:?}, {:?}): {}", index, v, err);
      }
    }
    out
  }
  /// See [glVertexAttrib4f](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib4f.xhtml)
  ///
  /// Fallbacks: VertexAttrib4fARB, VertexAttrib4fNV
  #[inline]
  pub unsafe fn glVertexAttrib4f(
    index: GLuint,
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    w: GLfloat,
  ) {
    trace!("glVertexAttrib4f");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib4f.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib4f is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat),
    >(p)(index, x, y, z, w);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib4f({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          index, x, y, z, w, err
        );
      }
    }
    out
  }
  /// See [glVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib.xhtml)
  ///
  /// Fallbacks: VertexAttrib4fvARB, VertexAttrib4fvNV
  #[inline]
  pub unsafe fn glVertexAttrib4fv(index: GLuint, v: *const GLfloat) {
    trace!("glVertexAttrib4fv");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttrib4fv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttrib4fv is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, *const GLfloat),
    >(p)(index, v);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttrib4fv({:?}, {:?}): {}", index, v, err);
      }
    }
    out
  }
  /// See [glVertexAttribDivisor](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttribDivisor.xhtml)
  ///
  /// Fallbacks: VertexAttribDivisorANGLE, VertexAttribDivisorARB,
  /// VertexAttribDivisorEXT, VertexAttribDivisorNV
  #[inline]
  pub unsafe fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) {
    trace!("glVertexAttribDivisor");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttribDivisor.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttribDivisor is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, GLuint)>(p)(
      index, divisor,
    );
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttribDivisor({:?}, {:?}): {}", index, divisor, err);
      }
    }
    out
  }
  /// See [glVertexAttribI4i](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttribI4i.xhtml)
  ///
  /// Fallbacks: VertexAttribI4iEXT
  #[inline]
  pub unsafe fn glVertexAttribI4i(
    index: GLuint,
    x: GLint,
    y: GLint,
    z: GLint,
    w: GLint,
  ) {
    trace!("glVertexAttribI4i");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttribI4i.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttribI4i is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLint, GLint, GLint, GLint),
    >(p)(index, x, y, z, w);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttribI4i({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          index, x, y, z, w, err
        );
      }
    }
    out
  }
  /// See [glVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib.xhtml)
  ///
  /// Fallbacks: VertexAttribI4ivEXT
  #[inline]
  pub unsafe fn glVertexAttribI4iv(index: GLuint, v: *const GLint) {
    trace!("glVertexAttribI4iv");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttribI4iv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttribI4iv is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, *const GLint)>(
      p,
    )(index, v);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttribI4iv({:?}, {:?}): {}", index, v, err);
      }
    }
    out
  }
  /// See [glVertexAttribI4ui](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttribI4ui.xhtml)
  ///
  /// Fallbacks: VertexAttribI4uiEXT
  #[inline]
  pub unsafe fn glVertexAttribI4ui(
    index: GLuint,
    x: GLuint,
    y: GLuint,
    z: GLuint,
    w: GLuint,
  ) {
    trace!("glVertexAttribI4ui");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttribI4ui.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttribI4ui is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint),
    >(p)(index, x, y, z, w);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttribI4ui({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          index, x, y, z, w, err
        );
      }
    }
    out
  }
  /// See [glVertexAttrib](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttrib.xhtml)
  ///
  /// Fallbacks: VertexAttribI4uivEXT
  #[inline]
  pub unsafe fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint) {
    trace!("glVertexAttribI4uiv");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttribI4uiv.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttribI4uiv is not loaded");
      }
      temp_p
    };
    let out = transmute::<*mut c_void, extern "system" fn(GLuint, *const GLuint)>(
      p,
    )(index, v);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttribI4uiv({:?}, {:?}): {}", index, v, err);
      }
    }
    out
  }
  /// See [glVertexAttribIPointer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttribIPointer.xhtml)
  ///
  /// Fallbacks: VertexAttribIPointerEXT
  #[inline]
  pub unsafe fn glVertexAttribIPointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
  ) {
    trace!("glVertexAttribIPointer");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttribIPointer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttribIPointer is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void),
    >(p)(index, size, type_, stride, pointer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttribIPointer({:?}, {:?}, {:?}, {:?}, {:?}): {}",
          index, size, type_, stride, pointer, err
        );
      }
    }
    out
  }
  /// See [glVertexAttribPointer](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glVertexAttribPointer.xhtml)
  ///
  /// Fallbacks: VertexAttribPointerARB
  #[inline]
  pub unsafe fn glVertexAttribPointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
  ) {
    trace!("glVertexAttribPointer");
    let p: *mut c_void = {
      let temp_p = storage::VertexAttribPointer.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glVertexAttribPointer is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(
        GLuint,
        GLint,
        GLenum,
        GLboolean,
        GLsizei,
        *const c_void,
      ),
    >(p)(index, size, type_, normalized, stride, pointer);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glVertexAttribPointer({:?}, {:?}, {:?}, {:?}, {:?}, {:?}): {}",
          index, size, type_, normalized, stride, pointer, err
        );
      }
    }
    out
  }
  /// See [glViewport](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glViewport.xhtml)
  #[inline]
  pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    trace!("glViewport");
    let p: *mut c_void = {
      let temp_p = storage::Viewport.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glViewport is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLint, GLint, GLsizei, GLsizei),
    >(p)(x, y, width, height);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glViewport({:?}, {:?}, {:?}, {:?}): {}",
          x, y, width, height, err
        );
      }
    }
    out
  }
  /// See [glWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glWaitSync.xhtml)
  ///
  /// Fallbacks: WaitSyncAPPLE
  #[inline]
  pub unsafe fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    trace!("glWaitSync");
    let p: *mut c_void = {
      let temp_p = storage::WaitSync.load(Ordering::Relaxed);
      if temp_p.is_null() {
        panic!("glWaitSync is not loaded");
      }
      temp_p
    };
    let out = transmute::<
      *mut c_void,
      extern "system" fn(GLsync, GLbitfield, GLuint64),
    >(p)(sync, flags, timeout);
    if cfg!(debug_assertions) {
      let err = glGetError();
      if err != GL_NO_ERROR {
        error!("glWaitSync({:?}, {:?}, {:?}): {}", sync, flags, timeout, err);
      }
    }
    out
  }
}

// KEEP AS PRIVATE.
mod storage {
  use super::*;
  pub static ActiveTexture: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static AttachShader: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BeginQuery: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BeginTransformFeedback: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static BindAttribLocation: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindBuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindBufferBase: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindBufferRange: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindFramebuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindRenderbuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindSampler: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindTexture: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BindTransformFeedback: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static BindVertexArray: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BlendColor: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BlendEquation: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BlendEquationSeparate: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static BlendFunc: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BlendFuncSeparate: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BlitFramebuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BufferData: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static BufferSubData: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CheckFramebufferStatus: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static Clear: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClearBufferfi: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClearBufferfv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClearBufferiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClearBufferuiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClearColor: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClearDepthf: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClearStencil: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ClientWaitSync: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ColorMask: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CompileShader: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CompressedTexImage2D: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static CompressedTexImage3D: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static CompressedTexSubImage2D: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static CompressedTexSubImage3D: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static CopyBufferSubData: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CopyTexImage2D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CopyTexSubImage2D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CopyTexSubImage3D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CreateProgram: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CreateShader: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static CullFace: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteBuffers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteFramebuffers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteProgram: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteQueries: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteRenderbuffers: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static DeleteSamplers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteShader: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteSync: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteTextures: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DeleteTransformFeedbacks: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static DeleteVertexArrays: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DepthFunc: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DepthMask: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DepthRangef: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DetachShader: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Disable: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DisableVertexAttribArray: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static DrawArrays: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DrawArraysInstanced: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static DrawBuffers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DrawElements: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static DrawElementsInstanced: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static DrawRangeElements: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Enable: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static EnableVertexAttribArray: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static EndQuery: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static EndTransformFeedback: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static FenceSync: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Finish: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Flush: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static FlushMappedBufferRange: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static FramebufferRenderbuffer: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static FramebufferTexture2D: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static FramebufferTextureLayer: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static FrontFace: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenBuffers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenFramebuffers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenQueries: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenRenderbuffers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenSamplers: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenTextures: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenTransformFeedbacks: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GenVertexArrays: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GenerateMipmap: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetActiveAttrib: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetActiveUniform: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetActiveUniformBlockName: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetActiveUniformBlockiv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetActiveUniformsiv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetAttachedShaders: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetAttribLocation: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetBooleanv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetBufferParameteri64v: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetBufferParameteriv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetBufferPointerv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetError: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetFloatv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetFragDataLocation: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetFramebufferAttachmentParameteriv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetInteger64i_v: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetInteger64v: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetIntegeri_v: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetIntegerv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetInternalformativ: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetProgramBinary: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetProgramInfoLog: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetProgramiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetQueryObjectuiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetQueryiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetRenderbufferParameteriv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetSamplerParameterfv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetSamplerParameteriv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetShaderInfoLog: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetShaderPrecisionFormat: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetShaderSource: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetShaderiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetString: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetStringi: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetSynciv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetTexParameterfv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetTexParameteriv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetTransformFeedbackVarying: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetUniformBlockIndex: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetUniformIndices: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetUniformLocation: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetUniformfv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetUniformiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetUniformuiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetVertexAttribIiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetVertexAttribIuiv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetVertexAttribPointerv: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static GetVertexAttribfv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static GetVertexAttribiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Hint: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static InvalidateFramebuffer: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static InvalidateSubFramebuffer: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static IsBuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsEnabled: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsFramebuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsProgram: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsQuery: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsRenderbuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsSampler: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsShader: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsSync: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsTexture: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static IsTransformFeedback: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static IsVertexArray: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static LineWidth: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static LinkProgram: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static MapBufferRange: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static PauseTransformFeedback: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static PixelStorei: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static PolygonOffset: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ProgramBinary: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ProgramParameteri: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ReadBuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ReadPixels: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ReleaseShaderCompiler: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static RenderbufferStorage: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static RenderbufferStorageMultisample: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static ResumeTransformFeedback: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static SampleCoverage: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static SamplerParameterf: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static SamplerParameterfv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static SamplerParameteri: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static SamplerParameteriv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Scissor: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ShaderBinary: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ShaderSource: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static StencilFunc: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static StencilFuncSeparate: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static StencilMask: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static StencilMaskSeparate: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static StencilOp: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static StencilOpSeparate: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexImage2D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexImage3D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexParameterf: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexParameterfv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexParameteri: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexParameteriv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexStorage2D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexStorage3D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexSubImage2D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TexSubImage3D: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static TransformFeedbackVaryings: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static Uniform1f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform1fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform1i: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform1iv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform1ui: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform1uiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform2f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform2fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform2i: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform2iv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform2ui: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform2uiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform3f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform3fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform3i: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform3iv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform3ui: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform3uiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform4f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform4fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform4i: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform4iv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform4ui: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static Uniform4uiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformBlockBinding: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static UniformMatrix2fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix2x3fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix2x4fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix3fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix3x2fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix3x4fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix4fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix4x2fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UniformMatrix4x3fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UnmapBuffer: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static UseProgram: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static ValidateProgram: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib1f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib1fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib2f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib2fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib3f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib3fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib4f: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttrib4fv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttribDivisor: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static VertexAttribI4i: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttribI4iv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttribI4ui: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttribI4uiv: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static VertexAttribIPointer: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static VertexAttribPointer: AtomicPtr<c_void> =
    AtomicPtr::new(null_mut());
  pub static Viewport: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
  pub static WaitSync: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
}

#[doc(hidden)]
pub mod ActiveTexture {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ActiveTexture.load(Ordering::Relaxed).is_null()
  }

  /// Load `ActiveTexture` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glActiveTexture\0", b"glActiveTextureARB\0"],
    ) {
      storage::ActiveTexture.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod AttachShader {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::AttachShader.load(Ordering::Relaxed).is_null()
  }

  /// Load `AttachShader` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glAttachShader\0", b"glAttachObjectARB\0"])
    {
      storage::AttachShader.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BeginQuery {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BeginQuery.load(Ordering::Relaxed).is_null()
  }

  /// Load `BeginQuery` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glBeginQuery\0", b"glBeginQueryARB\0"])
    {
      storage::BeginQuery.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BeginTransformFeedback {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BeginTransformFeedback.load(Ordering::Relaxed).is_null()
  }

  /// Load `BeginTransformFeedback` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glBeginTransformFeedback\0",
        b"glBeginTransformFeedbackEXT\0",
        b"glBeginTransformFeedbackNV\0",
      ],
    ) {
      storage::BeginTransformFeedback.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindAttribLocation {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindAttribLocation.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindAttribLocation` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glBindAttribLocation\0", b"glBindAttribLocationARB\0"],
    ) {
      storage::BindAttribLocation.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindBuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindBuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindBuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glBindBuffer\0", b"glBindBufferARB\0"])
    {
      storage::BindBuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindBufferBase {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindBufferBase.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindBufferBase` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glBindBufferBase\0",
        b"glBindBufferBaseEXT\0",
        b"glBindBufferBaseNV\0",
      ],
    ) {
      storage::BindBufferBase.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindBufferRange {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindBufferRange.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindBufferRange` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glBindBufferRange\0",
        b"glBindBufferRangeEXT\0",
        b"glBindBufferRangeNV\0",
      ],
    ) {
      storage::BindBufferRange.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindFramebuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindFramebuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindFramebuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glBindFramebuffer\0"]) {
      storage::BindFramebuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindRenderbuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindRenderbuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindRenderbuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glBindRenderbuffer\0"]) {
      storage::BindRenderbuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindSampler {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindSampler.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindSampler` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glBindSampler\0"]) {
      storage::BindSampler.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindTexture {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindTexture.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindTexture` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glBindTexture\0", b"glBindTextureEXT\0"])
    {
      storage::BindTexture.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindTransformFeedback {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindTransformFeedback.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindTransformFeedback` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glBindTransformFeedback\0"])
    {
      storage::BindTransformFeedback.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BindVertexArray {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BindVertexArray.load(Ordering::Relaxed).is_null()
  }

  /// Load `BindVertexArray` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glBindVertexArray\0", b"glBindVertexArrayOES\0"],
    ) {
      storage::BindVertexArray.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BlendColor {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BlendColor.load(Ordering::Relaxed).is_null()
  }

  /// Load `BlendColor` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glBlendColor\0", b"glBlendColorEXT\0"])
    {
      storage::BlendColor.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BlendEquation {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BlendEquation.load(Ordering::Relaxed).is_null()
  }

  /// Load `BlendEquation` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glBlendEquation\0", b"glBlendEquationEXT\0"],
    ) {
      storage::BlendEquation.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BlendEquationSeparate {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BlendEquationSeparate.load(Ordering::Relaxed).is_null()
  }

  /// Load `BlendEquationSeparate` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glBlendEquationSeparate\0", b"glBlendEquationSeparateEXT\0"],
    ) {
      storage::BlendEquationSeparate.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BlendFunc {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BlendFunc.load(Ordering::Relaxed).is_null()
  }

  /// Load `BlendFunc` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glBlendFunc\0"]) {
      storage::BlendFunc.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BlendFuncSeparate {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BlendFuncSeparate.load(Ordering::Relaxed).is_null()
  }

  /// Load `BlendFuncSeparate` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glBlendFuncSeparate\0",
        b"glBlendFuncSeparateEXT\0",
        b"glBlendFuncSeparateINGR\0",
      ],
    ) {
      storage::BlendFuncSeparate.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BlitFramebuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BlitFramebuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `BlitFramebuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glBlitFramebuffer\0",
        b"glBlitFramebufferEXT\0",
        b"glBlitFramebufferNV\0",
      ],
    ) {
      storage::BlitFramebuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BufferData {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BufferData.load(Ordering::Relaxed).is_null()
  }

  /// Load `BufferData` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glBufferData\0", b"glBufferDataARB\0"])
    {
      storage::BufferData.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod BufferSubData {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::BufferSubData.load(Ordering::Relaxed).is_null()
  }

  /// Load `BufferSubData` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glBufferSubData\0", b"glBufferSubDataARB\0"],
    ) {
      storage::BufferSubData.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CheckFramebufferStatus {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CheckFramebufferStatus.load(Ordering::Relaxed).is_null()
  }

  /// Load `CheckFramebufferStatus` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCheckFramebufferStatus\0", b"glCheckFramebufferStatusEXT\0"],
    ) {
      storage::CheckFramebufferStatus.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Clear {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Clear.load(Ordering::Relaxed).is_null()
  }

  /// Load `Clear` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glClear\0"]) {
      storage::Clear.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClearBufferfi {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClearBufferfi.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClearBufferfi` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glClearBufferfi\0"]) {
      storage::ClearBufferfi.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClearBufferfv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClearBufferfv.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClearBufferfv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glClearBufferfv\0"]) {
      storage::ClearBufferfv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClearBufferiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClearBufferiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClearBufferiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glClearBufferiv\0"]) {
      storage::ClearBufferiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClearBufferuiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClearBufferuiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClearBufferuiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glClearBufferuiv\0"]) {
      storage::ClearBufferuiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClearColor {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClearColor.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClearColor` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glClearColor\0"]) {
      storage::ClearColor.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClearDepthf {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClearDepthf.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClearDepthf` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glClearDepthf\0", b"glClearDepthfOES\0"])
    {
      storage::ClearDepthf.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClearStencil {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClearStencil.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClearStencil` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glClearStencil\0"]) {
      storage::ClearStencil.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ClientWaitSync {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ClientWaitSync.load(Ordering::Relaxed).is_null()
  }

  /// Load `ClientWaitSync` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glClientWaitSync\0", b"glClientWaitSyncAPPLE\0"],
    ) {
      storage::ClientWaitSync.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ColorMask {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ColorMask.load(Ordering::Relaxed).is_null()
  }

  /// Load `ColorMask` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glColorMask\0"]) {
      storage::ColorMask.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CompileShader {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CompileShader.load(Ordering::Relaxed).is_null()
  }

  /// Load `CompileShader` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCompileShader\0", b"glCompileShaderARB\0"],
    ) {
      storage::CompileShader.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CompressedTexImage2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CompressedTexImage2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `CompressedTexImage2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCompressedTexImage2D\0", b"glCompressedTexImage2DARB\0"],
    ) {
      storage::CompressedTexImage2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CompressedTexImage3D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CompressedTexImage3D.load(Ordering::Relaxed).is_null()
  }

  /// Load `CompressedTexImage3D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCompressedTexImage3D\0", b"glCompressedTexImage3DARB\0"],
    ) {
      storage::CompressedTexImage3D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CompressedTexSubImage2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CompressedTexSubImage2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `CompressedTexSubImage2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCompressedTexSubImage2D\0", b"glCompressedTexSubImage2DARB\0"],
    ) {
      storage::CompressedTexSubImage2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CompressedTexSubImage3D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CompressedTexSubImage3D.load(Ordering::Relaxed).is_null()
  }

  /// Load `CompressedTexSubImage3D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCompressedTexSubImage3D\0", b"glCompressedTexSubImage3DARB\0"],
    ) {
      storage::CompressedTexSubImage3D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CopyBufferSubData {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CopyBufferSubData.load(Ordering::Relaxed).is_null()
  }

  /// Load `CopyBufferSubData` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCopyBufferSubData\0", b"glCopyBufferSubDataNV\0"],
    ) {
      storage::CopyBufferSubData.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CopyTexImage2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CopyTexImage2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `CopyTexImage2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCopyTexImage2D\0", b"glCopyTexImage2DEXT\0"],
    ) {
      storage::CopyTexImage2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CopyTexSubImage2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CopyTexSubImage2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `CopyTexSubImage2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCopyTexSubImage2D\0", b"glCopyTexSubImage2DEXT\0"],
    ) {
      storage::CopyTexSubImage2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CopyTexSubImage3D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CopyTexSubImage3D.load(Ordering::Relaxed).is_null()
  }

  /// Load `CopyTexSubImage3D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCopyTexSubImage3D\0", b"glCopyTexSubImage3DEXT\0"],
    ) {
      storage::CopyTexSubImage3D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CreateProgram {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CreateProgram.load(Ordering::Relaxed).is_null()
  }

  /// Load `CreateProgram` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCreateProgram\0", b"glCreateProgramObjectARB\0"],
    ) {
      storage::CreateProgram.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CreateShader {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CreateShader.load(Ordering::Relaxed).is_null()
  }

  /// Load `CreateShader` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glCreateShader\0", b"glCreateShaderObjectARB\0"],
    ) {
      storage::CreateShader.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod CullFace {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::CullFace.load(Ordering::Relaxed).is_null()
  }

  /// Load `CullFace` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glCullFace\0"]) {
      storage::CullFace.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteBuffers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteBuffers.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteBuffers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glDeleteBuffers\0", b"glDeleteBuffersARB\0"],
    ) {
      storage::DeleteBuffers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteFramebuffers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteFramebuffers.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteFramebuffers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glDeleteFramebuffers\0", b"glDeleteFramebuffersEXT\0"],
    ) {
      storage::DeleteFramebuffers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteProgram {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteProgram.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteProgram` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDeleteProgram\0"]) {
      storage::DeleteProgram.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteQueries {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteQueries.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteQueries` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glDeleteQueries\0", b"glDeleteQueriesARB\0"],
    ) {
      storage::DeleteQueries.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteRenderbuffers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteRenderbuffers.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteRenderbuffers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glDeleteRenderbuffers\0", b"glDeleteRenderbuffersEXT\0"],
    ) {
      storage::DeleteRenderbuffers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteSamplers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteSamplers.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteSamplers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDeleteSamplers\0"]) {
      storage::DeleteSamplers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteShader {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteShader.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteShader` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDeleteShader\0"]) {
      storage::DeleteShader.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteSync {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteSync.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteSync` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glDeleteSync\0", b"glDeleteSyncAPPLE\0"])
    {
      storage::DeleteSync.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteTextures {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteTextures.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteTextures` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDeleteTextures\0"]) {
      storage::DeleteTextures.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteTransformFeedbacks {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteTransformFeedbacks.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteTransformFeedbacks` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glDeleteTransformFeedbacks\0", b"glDeleteTransformFeedbacksNV\0"],
    ) {
      storage::DeleteTransformFeedbacks.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DeleteVertexArrays {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DeleteVertexArrays.load(Ordering::Relaxed).is_null()
  }

  /// Load `DeleteVertexArrays` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glDeleteVertexArrays\0",
        b"glDeleteVertexArraysAPPLE\0",
        b"glDeleteVertexArraysOES\0",
      ],
    ) {
      storage::DeleteVertexArrays.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DepthFunc {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DepthFunc.load(Ordering::Relaxed).is_null()
  }

  /// Load `DepthFunc` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDepthFunc\0"]) {
      storage::DepthFunc.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DepthMask {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DepthMask.load(Ordering::Relaxed).is_null()
  }

  /// Load `DepthMask` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDepthMask\0"]) {
      storage::DepthMask.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DepthRangef {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DepthRangef.load(Ordering::Relaxed).is_null()
  }

  /// Load `DepthRangef` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glDepthRangef\0", b"glDepthRangefOES\0"])
    {
      storage::DepthRangef.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DetachShader {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DetachShader.load(Ordering::Relaxed).is_null()
  }

  /// Load `DetachShader` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glDetachShader\0", b"glDetachObjectARB\0"])
    {
      storage::DetachShader.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Disable {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Disable.load(Ordering::Relaxed).is_null()
  }

  /// Load `Disable` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDisable\0"]) {
      storage::Disable.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DisableVertexAttribArray {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DisableVertexAttribArray.load(Ordering::Relaxed).is_null()
  }

  /// Load `DisableVertexAttribArray` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glDisableVertexAttribArray\0", b"glDisableVertexAttribArrayARB\0"],
    ) {
      storage::DisableVertexAttribArray.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DrawArrays {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DrawArrays.load(Ordering::Relaxed).is_null()
  }

  /// Load `DrawArrays` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glDrawArrays\0", b"glDrawArraysEXT\0"])
    {
      storage::DrawArrays.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DrawArraysInstanced {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DrawArraysInstanced.load(Ordering::Relaxed).is_null()
  }

  /// Load `DrawArraysInstanced` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glDrawArraysInstanced\0",
        b"glDrawArraysInstancedANGLE\0",
        b"glDrawArraysInstancedARB\0",
        b"glDrawArraysInstancedEXT\0",
        b"glDrawArraysInstancedNV\0",
      ],
    ) {
      storage::DrawArraysInstanced.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DrawBuffers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DrawBuffers.load(Ordering::Relaxed).is_null()
  }

  /// Load `DrawBuffers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glDrawBuffers\0",
        b"glDrawBuffersARB\0",
        b"glDrawBuffersATI\0",
        b"glDrawBuffersEXT\0",
      ],
    ) {
      storage::DrawBuffers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DrawElements {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DrawElements.load(Ordering::Relaxed).is_null()
  }

  /// Load `DrawElements` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glDrawElements\0"]) {
      storage::DrawElements.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DrawElementsInstanced {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DrawElementsInstanced.load(Ordering::Relaxed).is_null()
  }

  /// Load `DrawElementsInstanced` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glDrawElementsInstanced\0",
        b"glDrawElementsInstancedANGLE\0",
        b"glDrawElementsInstancedARB\0",
        b"glDrawElementsInstancedEXT\0",
        b"glDrawElementsInstancedNV\0",
      ],
    ) {
      storage::DrawElementsInstanced.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod DrawRangeElements {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::DrawRangeElements.load(Ordering::Relaxed).is_null()
  }

  /// Load `DrawRangeElements` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glDrawRangeElements\0", b"glDrawRangeElementsEXT\0"],
    ) {
      storage::DrawRangeElements.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Enable {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Enable.load(Ordering::Relaxed).is_null()
  }

  /// Load `Enable` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glEnable\0"]) {
      storage::Enable.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod EnableVertexAttribArray {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::EnableVertexAttribArray.load(Ordering::Relaxed).is_null()
  }

  /// Load `EnableVertexAttribArray` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glEnableVertexAttribArray\0", b"glEnableVertexAttribArrayARB\0"],
    ) {
      storage::EnableVertexAttribArray.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod EndQuery {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::EndQuery.load(Ordering::Relaxed).is_null()
  }

  /// Load `EndQuery` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glEndQuery\0", b"glEndQueryARB\0"])
    {
      storage::EndQuery.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod EndTransformFeedback {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::EndTransformFeedback.load(Ordering::Relaxed).is_null()
  }

  /// Load `EndTransformFeedback` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glEndTransformFeedback\0",
        b"glEndTransformFeedbackEXT\0",
        b"glEndTransformFeedbackNV\0",
      ],
    ) {
      storage::EndTransformFeedback.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod FenceSync {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::FenceSync.load(Ordering::Relaxed).is_null()
  }

  /// Load `FenceSync` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glFenceSync\0", b"glFenceSyncAPPLE\0"])
    {
      storage::FenceSync.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Finish {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Finish.load(Ordering::Relaxed).is_null()
  }

  /// Load `Finish` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glFinish\0"]) {
      storage::Finish.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Flush {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Flush.load(Ordering::Relaxed).is_null()
  }

  /// Load `Flush` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glFlush\0"]) {
      storage::Flush.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod FlushMappedBufferRange {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::FlushMappedBufferRange.load(Ordering::Relaxed).is_null()
  }

  /// Load `FlushMappedBufferRange` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glFlushMappedBufferRange\0",
        b"glFlushMappedBufferRangeAPPLE\0",
        b"glFlushMappedBufferRangeEXT\0",
      ],
    ) {
      storage::FlushMappedBufferRange.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod FramebufferRenderbuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::FramebufferRenderbuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `FramebufferRenderbuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glFramebufferRenderbuffer\0", b"glFramebufferRenderbufferEXT\0"],
    ) {
      storage::FramebufferRenderbuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod FramebufferTexture2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::FramebufferTexture2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `FramebufferTexture2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glFramebufferTexture2D\0", b"glFramebufferTexture2DEXT\0"],
    ) {
      storage::FramebufferTexture2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod FramebufferTextureLayer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::FramebufferTextureLayer.load(Ordering::Relaxed).is_null()
  }

  /// Load `FramebufferTextureLayer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glFramebufferTextureLayer\0",
        b"glFramebufferTextureLayerARB\0",
        b"glFramebufferTextureLayerEXT\0",
      ],
    ) {
      storage::FramebufferTextureLayer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod FrontFace {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::FrontFace.load(Ordering::Relaxed).is_null()
  }

  /// Load `FrontFace` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glFrontFace\0"]) {
      storage::FrontFace.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenBuffers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenBuffers.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenBuffers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGenBuffers\0", b"glGenBuffersARB\0"])
    {
      storage::GenBuffers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenFramebuffers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenFramebuffers.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenFramebuffers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGenFramebuffers\0", b"glGenFramebuffersEXT\0"],
    ) {
      storage::GenFramebuffers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenQueries {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenQueries.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenQueries` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGenQueries\0", b"glGenQueriesARB\0"])
    {
      storage::GenQueries.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenRenderbuffers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenRenderbuffers.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenRenderbuffers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGenRenderbuffers\0", b"glGenRenderbuffersEXT\0"],
    ) {
      storage::GenRenderbuffers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenSamplers {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenSamplers.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenSamplers` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGenSamplers\0"]) {
      storage::GenSamplers.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenTextures {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenTextures.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenTextures` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGenTextures\0"]) {
      storage::GenTextures.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenTransformFeedbacks {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenTransformFeedbacks.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenTransformFeedbacks` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGenTransformFeedbacks\0", b"glGenTransformFeedbacksNV\0"],
    ) {
      storage::GenTransformFeedbacks.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenVertexArrays {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenVertexArrays.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenVertexArrays` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGenVertexArrays\0",
        b"glGenVertexArraysAPPLE\0",
        b"glGenVertexArraysOES\0",
      ],
    ) {
      storage::GenVertexArrays.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GenerateMipmap {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GenerateMipmap.load(Ordering::Relaxed).is_null()
  }

  /// Load `GenerateMipmap` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGenerateMipmap\0", b"glGenerateMipmapEXT\0"],
    ) {
      storage::GenerateMipmap.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetActiveAttrib {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetActiveAttrib.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetActiveAttrib` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetActiveAttrib\0", b"glGetActiveAttribARB\0"],
    ) {
      storage::GetActiveAttrib.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetActiveUniform {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetActiveUniform.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetActiveUniform` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetActiveUniform\0", b"glGetActiveUniformARB\0"],
    ) {
      storage::GetActiveUniform.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetActiveUniformBlockName {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetActiveUniformBlockName.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetActiveUniformBlockName` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGetActiveUniformBlockName\0"])
    {
      storage::GetActiveUniformBlockName.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetActiveUniformBlockiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetActiveUniformBlockiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetActiveUniformBlockiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGetActiveUniformBlockiv\0"])
    {
      storage::GetActiveUniformBlockiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetActiveUniformsiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetActiveUniformsiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetActiveUniformsiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetActiveUniformsiv\0"]) {
      storage::GetActiveUniformsiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetAttachedShaders {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetAttachedShaders.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetAttachedShaders` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetAttachedShaders\0"]) {
      storage::GetAttachedShaders.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetAttribLocation {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetAttribLocation.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetAttribLocation` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetAttribLocation\0", b"glGetAttribLocationARB\0"],
    ) {
      storage::GetAttribLocation.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetBooleanv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetBooleanv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetBooleanv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetBooleanv\0"]) {
      storage::GetBooleanv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetBufferParameteri64v {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetBufferParameteri64v.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetBufferParameteri64v` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetBufferParameteri64v\0"])
    {
      storage::GetBufferParameteri64v.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetBufferParameteriv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetBufferParameteriv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetBufferParameteriv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetBufferParameteriv\0", b"glGetBufferParameterivARB\0"],
    ) {
      storage::GetBufferParameteriv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetBufferPointerv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetBufferPointerv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetBufferPointerv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGetBufferPointerv\0",
        b"glGetBufferPointervARB\0",
        b"glGetBufferPointervOES\0",
      ],
    ) {
      storage::GetBufferPointerv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetError {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetError.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetError` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetError\0"]) {
      storage::GetError.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetFloatv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetFloatv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetFloatv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetFloatv\0"]) {
      storage::GetFloatv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetFragDataLocation {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetFragDataLocation.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetFragDataLocation` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetFragDataLocation\0", b"glGetFragDataLocationEXT\0"],
    ) {
      storage::GetFragDataLocation.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetFramebufferAttachmentParameteriv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetFramebufferAttachmentParameteriv
      .load(Ordering::Relaxed)
      .is_null()
  }

  /// Load `GetFramebufferAttachmentParameteriv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGetFramebufferAttachmentParameteriv\0",
        b"glGetFramebufferAttachmentParameterivEXT\0",
      ],
    ) {
      storage::GetFramebufferAttachmentParameteriv
        .store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetInteger64i_v {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetInteger64i_v.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetInteger64i_v` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetInteger64i_v\0"]) {
      storage::GetInteger64i_v.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetInteger64v {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetInteger64v.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetInteger64v` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetInteger64v\0", b"glGetInteger64vAPPLE\0"],
    ) {
      storage::GetInteger64v.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetIntegeri_v {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetIntegeri_v.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetIntegeri_v` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetIntegeri_v\0", b"glGetIntegerIndexedvEXT\0"],
    ) {
      storage::GetIntegeri_v.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetIntegerv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetIntegerv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetIntegerv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetIntegerv\0"]) {
      storage::GetIntegerv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetInternalformativ {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetInternalformativ.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetInternalformativ` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetInternalformativ\0"]) {
      storage::GetInternalformativ.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetProgramBinary {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetProgramBinary.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetProgramBinary` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetProgramBinary\0", b"glGetProgramBinaryOES\0"],
    ) {
      storage::GetProgramBinary.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetProgramInfoLog {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetProgramInfoLog.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetProgramInfoLog` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetProgramInfoLog\0"]) {
      storage::GetProgramInfoLog.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetProgramiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetProgramiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetProgramiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetProgramiv\0"]) {
      storage::GetProgramiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetQueryObjectuiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetQueryObjectuiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetQueryObjectuiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetQueryObjectuiv\0", b"glGetQueryObjectuivARB\0"],
    ) {
      storage::GetQueryObjectuiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetQueryiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetQueryiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetQueryiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGetQueryiv\0", b"glGetQueryivARB\0"])
    {
      storage::GetQueryiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetRenderbufferParameteriv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetRenderbufferParameteriv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetRenderbufferParameteriv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGetRenderbufferParameteriv\0",
        b"glGetRenderbufferParameterivEXT\0",
      ],
    ) {
      storage::GetRenderbufferParameteriv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetSamplerParameterfv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetSamplerParameterfv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetSamplerParameterfv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetSamplerParameterfv\0"])
    {
      storage::GetSamplerParameterfv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetSamplerParameteriv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetSamplerParameteriv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetSamplerParameteriv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetSamplerParameteriv\0"])
    {
      storage::GetSamplerParameteriv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetShaderInfoLog {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetShaderInfoLog.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetShaderInfoLog` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetShaderInfoLog\0"]) {
      storage::GetShaderInfoLog.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetShaderPrecisionFormat {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetShaderPrecisionFormat.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetShaderPrecisionFormat` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGetShaderPrecisionFormat\0"])
    {
      storage::GetShaderPrecisionFormat.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetShaderSource {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetShaderSource.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetShaderSource` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetShaderSource\0", b"glGetShaderSourceARB\0"],
    ) {
      storage::GetShaderSource.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetShaderiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetShaderiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetShaderiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetShaderiv\0"]) {
      storage::GetShaderiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetString {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetString.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetString` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetString\0"]) {
      storage::GetString.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetStringi {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetStringi.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetStringi` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetStringi\0"]) {
      storage::GetStringi.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetSynciv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetSynciv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetSynciv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGetSynciv\0", b"glGetSyncivAPPLE\0"])
    {
      storage::GetSynciv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetTexParameterfv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetTexParameterfv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetTexParameterfv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetTexParameterfv\0"]) {
      storage::GetTexParameterfv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetTexParameteriv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetTexParameteriv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetTexParameteriv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetTexParameteriv\0"]) {
      storage::GetTexParameteriv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetTransformFeedbackVarying {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetTransformFeedbackVarying.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetTransformFeedbackVarying` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGetTransformFeedbackVarying\0",
        b"glGetTransformFeedbackVaryingEXT\0",
      ],
    ) {
      storage::GetTransformFeedbackVarying.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetUniformBlockIndex {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetUniformBlockIndex.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetUniformBlockIndex` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetUniformBlockIndex\0"]) {
      storage::GetUniformBlockIndex.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetUniformIndices {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetUniformIndices.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetUniformIndices` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glGetUniformIndices\0"]) {
      storage::GetUniformIndices.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetUniformLocation {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetUniformLocation.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetUniformLocation` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetUniformLocation\0", b"glGetUniformLocationARB\0"],
    ) {
      storage::GetUniformLocation.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetUniformfv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetUniformfv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetUniformfv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGetUniformfv\0", b"glGetUniformfvARB\0"])
    {
      storage::GetUniformfv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetUniformiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetUniformiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetUniformiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glGetUniformiv\0", b"glGetUniformivARB\0"])
    {
      storage::GetUniformiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetUniformuiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetUniformuiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetUniformuiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetUniformuiv\0", b"glGetUniformuivEXT\0"],
    ) {
      storage::GetUniformuiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetVertexAttribIiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetVertexAttribIiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetVertexAttribIiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetVertexAttribIiv\0", b"glGetVertexAttribIivEXT\0"],
    ) {
      storage::GetVertexAttribIiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetVertexAttribIuiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetVertexAttribIuiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetVertexAttribIuiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glGetVertexAttribIuiv\0", b"glGetVertexAttribIuivEXT\0"],
    ) {
      storage::GetVertexAttribIuiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetVertexAttribPointerv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetVertexAttribPointerv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetVertexAttribPointerv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGetVertexAttribPointerv\0",
        b"glGetVertexAttribPointervARB\0",
        b"glGetVertexAttribPointervNV\0",
      ],
    ) {
      storage::GetVertexAttribPointerv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetVertexAttribfv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetVertexAttribfv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetVertexAttribfv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGetVertexAttribfv\0",
        b"glGetVertexAttribfvARB\0",
        b"glGetVertexAttribfvNV\0",
      ],
    ) {
      storage::GetVertexAttribfv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod GetVertexAttribiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::GetVertexAttribiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `GetVertexAttribiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glGetVertexAttribiv\0",
        b"glGetVertexAttribivARB\0",
        b"glGetVertexAttribivNV\0",
      ],
    ) {
      storage::GetVertexAttribiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Hint {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Hint.load(Ordering::Relaxed).is_null()
  }

  /// Load `Hint` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glHint\0"]) {
      storage::Hint.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod InvalidateFramebuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::InvalidateFramebuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `InvalidateFramebuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glInvalidateFramebuffer\0"])
    {
      storage::InvalidateFramebuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod InvalidateSubFramebuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::InvalidateSubFramebuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `InvalidateSubFramebuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glInvalidateSubFramebuffer\0"])
    {
      storage::InvalidateSubFramebuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsBuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsBuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsBuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glIsBuffer\0", b"glIsBufferARB\0"])
    {
      storage::IsBuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsEnabled {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsEnabled.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsEnabled` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glIsEnabled\0"]) {
      storage::IsEnabled.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsFramebuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsFramebuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsFramebuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glIsFramebuffer\0", b"glIsFramebufferEXT\0"],
    ) {
      storage::IsFramebuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsProgram {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsProgram.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsProgram` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glIsProgram\0"]) {
      storage::IsProgram.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsQuery {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsQuery.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsQuery` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glIsQuery\0", b"glIsQueryARB\0"])
    {
      storage::IsQuery.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsRenderbuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsRenderbuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsRenderbuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glIsRenderbuffer\0", b"glIsRenderbufferEXT\0"],
    ) {
      storage::IsRenderbuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsSampler {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsSampler.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsSampler` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glIsSampler\0"]) {
      storage::IsSampler.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsShader {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsShader.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsShader` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glIsShader\0"]) {
      storage::IsShader.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsSync {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsSync.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsSync` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glIsSync\0", b"glIsSyncAPPLE\0"])
    {
      storage::IsSync.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsTexture {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsTexture.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsTexture` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glIsTexture\0"]) {
      storage::IsTexture.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsTransformFeedback {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsTransformFeedback.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsTransformFeedback` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glIsTransformFeedback\0", b"glIsTransformFeedbackNV\0"],
    ) {
      storage::IsTransformFeedback.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod IsVertexArray {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::IsVertexArray.load(Ordering::Relaxed).is_null()
  }

  /// Load `IsVertexArray` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glIsVertexArray\0",
        b"glIsVertexArrayAPPLE\0",
        b"glIsVertexArrayOES\0",
      ],
    ) {
      storage::IsVertexArray.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod LineWidth {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::LineWidth.load(Ordering::Relaxed).is_null()
  }

  /// Load `LineWidth` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glLineWidth\0"]) {
      storage::LineWidth.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod LinkProgram {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::LinkProgram.load(Ordering::Relaxed).is_null()
  }

  /// Load `LinkProgram` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glLinkProgram\0", b"glLinkProgramARB\0"])
    {
      storage::LinkProgram.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod MapBufferRange {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::MapBufferRange.load(Ordering::Relaxed).is_null()
  }

  /// Load `MapBufferRange` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glMapBufferRange\0", b"glMapBufferRangeEXT\0"],
    ) {
      storage::MapBufferRange.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod PauseTransformFeedback {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::PauseTransformFeedback.load(Ordering::Relaxed).is_null()
  }

  /// Load `PauseTransformFeedback` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glPauseTransformFeedback\0", b"glPauseTransformFeedbackNV\0"],
    ) {
      storage::PauseTransformFeedback.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod PixelStorei {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::PixelStorei.load(Ordering::Relaxed).is_null()
  }

  /// Load `PixelStorei` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glPixelStorei\0"]) {
      storage::PixelStorei.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod PolygonOffset {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::PolygonOffset.load(Ordering::Relaxed).is_null()
  }

  /// Load `PolygonOffset` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glPolygonOffset\0"]) {
      storage::PolygonOffset.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ProgramBinary {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ProgramBinary.load(Ordering::Relaxed).is_null()
  }

  /// Load `ProgramBinary` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glProgramBinary\0", b"glProgramBinaryOES\0"],
    ) {
      storage::ProgramBinary.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ProgramParameteri {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ProgramParameteri.load(Ordering::Relaxed).is_null()
  }

  /// Load `ProgramParameteri` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glProgramParameteri\0",
        b"glProgramParameteriARB\0",
        b"glProgramParameteriEXT\0",
      ],
    ) {
      storage::ProgramParameteri.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ReadBuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ReadBuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `ReadBuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glReadBuffer\0"]) {
      storage::ReadBuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ReadPixels {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ReadPixels.load(Ordering::Relaxed).is_null()
  }

  /// Load `ReadPixels` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glReadPixels\0"]) {
      storage::ReadPixels.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ReleaseShaderCompiler {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ReleaseShaderCompiler.load(Ordering::Relaxed).is_null()
  }

  /// Load `ReleaseShaderCompiler` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glReleaseShaderCompiler\0"])
    {
      storage::ReleaseShaderCompiler.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod RenderbufferStorage {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::RenderbufferStorage.load(Ordering::Relaxed).is_null()
  }

  /// Load `RenderbufferStorage` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glRenderbufferStorage\0", b"glRenderbufferStorageEXT\0"],
    ) {
      storage::RenderbufferStorage.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod RenderbufferStorageMultisample {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::RenderbufferStorageMultisample.load(Ordering::Relaxed).is_null()
  }

  /// Load `RenderbufferStorageMultisample` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glRenderbufferStorageMultisample\0",
        b"glRenderbufferStorageMultisampleEXT\0",
        b"glRenderbufferStorageMultisampleNV\0",
      ],
    ) {
      storage::RenderbufferStorageMultisample
        .store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ResumeTransformFeedback {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ResumeTransformFeedback.load(Ordering::Relaxed).is_null()
  }

  /// Load `ResumeTransformFeedback` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glResumeTransformFeedback\0", b"glResumeTransformFeedbackNV\0"],
    ) {
      storage::ResumeTransformFeedback.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod SampleCoverage {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::SampleCoverage.load(Ordering::Relaxed).is_null()
  }

  /// Load `SampleCoverage` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glSampleCoverage\0", b"glSampleCoverageARB\0"],
    ) {
      storage::SampleCoverage.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod SamplerParameterf {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::SamplerParameterf.load(Ordering::Relaxed).is_null()
  }

  /// Load `SamplerParameterf` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glSamplerParameterf\0"]) {
      storage::SamplerParameterf.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod SamplerParameterfv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::SamplerParameterfv.load(Ordering::Relaxed).is_null()
  }

  /// Load `SamplerParameterfv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glSamplerParameterfv\0"]) {
      storage::SamplerParameterfv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod SamplerParameteri {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::SamplerParameteri.load(Ordering::Relaxed).is_null()
  }

  /// Load `SamplerParameteri` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glSamplerParameteri\0"]) {
      storage::SamplerParameteri.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod SamplerParameteriv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::SamplerParameteriv.load(Ordering::Relaxed).is_null()
  }

  /// Load `SamplerParameteriv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glSamplerParameteriv\0"]) {
      storage::SamplerParameteriv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Scissor {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Scissor.load(Ordering::Relaxed).is_null()
  }

  /// Load `Scissor` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glScissor\0"]) {
      storage::Scissor.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ShaderBinary {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ShaderBinary.load(Ordering::Relaxed).is_null()
  }

  /// Load `ShaderBinary` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glShaderBinary\0"]) {
      storage::ShaderBinary.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ShaderSource {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ShaderSource.load(Ordering::Relaxed).is_null()
  }

  /// Load `ShaderSource` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glShaderSource\0", b"glShaderSourceARB\0"])
    {
      storage::ShaderSource.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod StencilFunc {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::StencilFunc.load(Ordering::Relaxed).is_null()
  }

  /// Load `StencilFunc` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glStencilFunc\0"]) {
      storage::StencilFunc.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod StencilFuncSeparate {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::StencilFuncSeparate.load(Ordering::Relaxed).is_null()
  }

  /// Load `StencilFuncSeparate` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glStencilFuncSeparate\0"]) {
      storage::StencilFuncSeparate.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod StencilMask {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::StencilMask.load(Ordering::Relaxed).is_null()
  }

  /// Load `StencilMask` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glStencilMask\0"]) {
      storage::StencilMask.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod StencilMaskSeparate {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::StencilMaskSeparate.load(Ordering::Relaxed).is_null()
  }

  /// Load `StencilMaskSeparate` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glStencilMaskSeparate\0"]) {
      storage::StencilMaskSeparate.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod StencilOp {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::StencilOp.load(Ordering::Relaxed).is_null()
  }

  /// Load `StencilOp` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glStencilOp\0"]) {
      storage::StencilOp.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod StencilOpSeparate {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::StencilOpSeparate.load(Ordering::Relaxed).is_null()
  }

  /// Load `StencilOpSeparate` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glStencilOpSeparate\0", b"glStencilOpSeparateATI\0"],
    ) {
      storage::StencilOpSeparate.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexImage2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexImage2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexImage2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glTexImage2D\0"]) {
      storage::TexImage2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexImage3D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexImage3D.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexImage3D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glTexImage3D\0", b"glTexImage3DEXT\0"])
    {
      storage::TexImage3D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexParameterf {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexParameterf.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexParameterf` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glTexParameterf\0"]) {
      storage::TexParameterf.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexParameterfv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexParameterfv.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexParameterfv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glTexParameterfv\0"]) {
      storage::TexParameterfv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexParameteri {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexParameteri.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexParameteri` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glTexParameteri\0"]) {
      storage::TexParameteri.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexParameteriv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexParameteriv.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexParameteriv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glTexParameteriv\0"]) {
      storage::TexParameteriv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexStorage2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexStorage2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexStorage2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glTexStorage2D\0", b"glTexStorage2DEXT\0"])
    {
      storage::TexStorage2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexStorage3D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexStorage3D.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexStorage3D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glTexStorage3D\0", b"glTexStorage3DEXT\0"])
    {
      storage::TexStorage3D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexSubImage2D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexSubImage2D.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexSubImage2D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glTexSubImage2D\0", b"glTexSubImage2DEXT\0"],
    ) {
      storage::TexSubImage2D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TexSubImage3D {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TexSubImage3D.load(Ordering::Relaxed).is_null()
  }

  /// Load `TexSubImage3D` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glTexSubImage3D\0", b"glTexSubImage3DEXT\0"],
    ) {
      storage::TexSubImage3D.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod TransformFeedbackVaryings {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::TransformFeedbackVaryings.load(Ordering::Relaxed).is_null()
  }

  /// Load `TransformFeedbackVaryings` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glTransformFeedbackVaryings\0", b"glTransformFeedbackVaryingsEXT\0"],
    ) {
      storage::TransformFeedbackVaryings.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform1f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform1f.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform1f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform1f\0", b"glUniform1fARB\0"])
    {
      storage::Uniform1f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform1fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform1fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform1fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform1fv\0", b"glUniform1fvARB\0"])
    {
      storage::Uniform1fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform1i {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform1i.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform1i` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform1i\0", b"glUniform1iARB\0"])
    {
      storage::Uniform1i.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform1iv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform1iv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform1iv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform1iv\0", b"glUniform1ivARB\0"])
    {
      storage::Uniform1iv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform1ui {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform1ui.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform1ui` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform1ui\0", b"glUniform1uiEXT\0"])
    {
      storage::Uniform1ui.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform1uiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform1uiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform1uiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform1uiv\0", b"glUniform1uivEXT\0"])
    {
      storage::Uniform1uiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform2f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform2f.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform2f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform2f\0", b"glUniform2fARB\0"])
    {
      storage::Uniform2f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform2fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform2fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform2fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform2fv\0", b"glUniform2fvARB\0"])
    {
      storage::Uniform2fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform2i {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform2i.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform2i` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform2i\0", b"glUniform2iARB\0"])
    {
      storage::Uniform2i.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform2iv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform2iv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform2iv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform2iv\0", b"glUniform2ivARB\0"])
    {
      storage::Uniform2iv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform2ui {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform2ui.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform2ui` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform2ui\0", b"glUniform2uiEXT\0"])
    {
      storage::Uniform2ui.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform2uiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform2uiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform2uiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform2uiv\0", b"glUniform2uivEXT\0"])
    {
      storage::Uniform2uiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform3f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform3f.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform3f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform3f\0", b"glUniform3fARB\0"])
    {
      storage::Uniform3f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform3fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform3fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform3fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform3fv\0", b"glUniform3fvARB\0"])
    {
      storage::Uniform3fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform3i {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform3i.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform3i` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform3i\0", b"glUniform3iARB\0"])
    {
      storage::Uniform3i.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform3iv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform3iv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform3iv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform3iv\0", b"glUniform3ivARB\0"])
    {
      storage::Uniform3iv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform3ui {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform3ui.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform3ui` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform3ui\0", b"glUniform3uiEXT\0"])
    {
      storage::Uniform3ui.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform3uiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform3uiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform3uiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform3uiv\0", b"glUniform3uivEXT\0"])
    {
      storage::Uniform3uiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform4f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform4f.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform4f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform4f\0", b"glUniform4fARB\0"])
    {
      storage::Uniform4f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform4fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform4fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform4fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform4fv\0", b"glUniform4fvARB\0"])
    {
      storage::Uniform4fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform4i {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform4i.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform4i` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform4i\0", b"glUniform4iARB\0"])
    {
      storage::Uniform4i.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform4iv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform4iv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform4iv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform4iv\0", b"glUniform4ivARB\0"])
    {
      storage::Uniform4iv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform4ui {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform4ui.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform4ui` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform4ui\0", b"glUniform4uiEXT\0"])
    {
      storage::Uniform4ui.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Uniform4uiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Uniform4uiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `Uniform4uiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glUniform4uiv\0", b"glUniform4uivEXT\0"])
    {
      storage::Uniform4uiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformBlockBinding {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformBlockBinding.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformBlockBinding` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glUniformBlockBinding\0"]) {
      storage::UniformBlockBinding.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix2fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix2fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix2fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix2fv\0", b"glUniformMatrix2fvARB\0"],
    ) {
      storage::UniformMatrix2fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix2x3fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix2x3fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix2x3fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix2x3fv\0", b"glUniformMatrix2x3fvNV\0"],
    ) {
      storage::UniformMatrix2x3fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix2x4fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix2x4fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix2x4fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix2x4fv\0", b"glUniformMatrix2x4fvNV\0"],
    ) {
      storage::UniformMatrix2x4fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix3fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix3fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix3fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix3fv\0", b"glUniformMatrix3fvARB\0"],
    ) {
      storage::UniformMatrix3fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix3x2fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix3x2fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix3x2fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix3x2fv\0", b"glUniformMatrix3x2fvNV\0"],
    ) {
      storage::UniformMatrix3x2fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix3x4fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix3x4fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix3x4fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix3x4fv\0", b"glUniformMatrix3x4fvNV\0"],
    ) {
      storage::UniformMatrix3x4fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix4fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix4fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix4fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix4fv\0", b"glUniformMatrix4fvARB\0"],
    ) {
      storage::UniformMatrix4fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix4x2fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix4x2fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix4x2fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix4x2fv\0", b"glUniformMatrix4x2fvNV\0"],
    ) {
      storage::UniformMatrix4x2fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UniformMatrix4x3fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UniformMatrix4x3fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `UniformMatrix4x3fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUniformMatrix4x3fv\0", b"glUniformMatrix4x3fvNV\0"],
    ) {
      storage::UniformMatrix4x3fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UnmapBuffer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UnmapBuffer.load(Ordering::Relaxed).is_null()
  }

  /// Load `UnmapBuffer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUnmapBuffer\0", b"glUnmapBufferARB\0", b"glUnmapBufferOES\0"],
    ) {
      storage::UnmapBuffer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod UseProgram {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::UseProgram.load(Ordering::Relaxed).is_null()
  }

  /// Load `UseProgram` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glUseProgram\0", b"glUseProgramObjectARB\0"],
    ) {
      storage::UseProgram.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod ValidateProgram {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::ValidateProgram.load(Ordering::Relaxed).is_null()
  }

  /// Load `ValidateProgram` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glValidateProgram\0", b"glValidateProgramARB\0"],
    ) {
      storage::ValidateProgram.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib1f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib1f.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib1f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib1f\0",
        b"glVertexAttrib1fARB\0",
        b"glVertexAttrib1fNV\0",
      ],
    ) {
      storage::VertexAttrib1f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib1fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib1fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib1fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib1fv\0",
        b"glVertexAttrib1fvARB\0",
        b"glVertexAttrib1fvNV\0",
      ],
    ) {
      storage::VertexAttrib1fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib2f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib2f.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib2f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib2f\0",
        b"glVertexAttrib2fARB\0",
        b"glVertexAttrib2fNV\0",
      ],
    ) {
      storage::VertexAttrib2f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib2fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib2fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib2fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib2fv\0",
        b"glVertexAttrib2fvARB\0",
        b"glVertexAttrib2fvNV\0",
      ],
    ) {
      storage::VertexAttrib2fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib3f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib3f.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib3f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib3f\0",
        b"glVertexAttrib3fARB\0",
        b"glVertexAttrib3fNV\0",
      ],
    ) {
      storage::VertexAttrib3f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib3fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib3fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib3fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib3fv\0",
        b"glVertexAttrib3fvARB\0",
        b"glVertexAttrib3fvNV\0",
      ],
    ) {
      storage::VertexAttrib3fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib4f {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib4f.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib4f` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib4f\0",
        b"glVertexAttrib4fARB\0",
        b"glVertexAttrib4fNV\0",
      ],
    ) {
      storage::VertexAttrib4f.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttrib4fv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttrib4fv.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttrib4fv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttrib4fv\0",
        b"glVertexAttrib4fvARB\0",
        b"glVertexAttrib4fvNV\0",
      ],
    ) {
      storage::VertexAttrib4fv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttribDivisor {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttribDivisor.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttribDivisor` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[
        b"glVertexAttribDivisor\0",
        b"glVertexAttribDivisorANGLE\0",
        b"glVertexAttribDivisorARB\0",
        b"glVertexAttribDivisorEXT\0",
        b"glVertexAttribDivisorNV\0",
      ],
    ) {
      storage::VertexAttribDivisor.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttribI4i {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttribI4i.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttribI4i` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glVertexAttribI4i\0", b"glVertexAttribI4iEXT\0"],
    ) {
      storage::VertexAttribI4i.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttribI4iv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttribI4iv.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttribI4iv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glVertexAttribI4iv\0", b"glVertexAttribI4ivEXT\0"],
    ) {
      storage::VertexAttribI4iv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttribI4ui {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttribI4ui.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttribI4ui` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glVertexAttribI4ui\0", b"glVertexAttribI4uiEXT\0"],
    ) {
      storage::VertexAttribI4ui.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttribI4uiv {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttribI4uiv.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttribI4uiv` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glVertexAttribI4uiv\0", b"glVertexAttribI4uivEXT\0"],
    ) {
      storage::VertexAttribI4uiv.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttribIPointer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttribIPointer.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttribIPointer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glVertexAttribIPointer\0", b"glVertexAttribIPointerEXT\0"],
    ) {
      storage::VertexAttribIPointer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod VertexAttribPointer {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::VertexAttribPointer.load(Ordering::Relaxed).is_null()
  }

  /// Load `VertexAttribPointer` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(
      &mut load_fn,
      &[b"glVertexAttribPointer\0", b"glVertexAttribPointerARB\0"],
    ) {
      storage::VertexAttribPointer.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod Viewport {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::Viewport.load(Ordering::Relaxed).is_null()
  }

  /// Load `Viewport` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gles_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) = meta_loader(&mut load_fn, &[b"glViewport\0"]) {
      storage::Viewport.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}

#[doc(hidden)]
pub mod WaitSync {
  use super::*;

  #[inline]
  pub fn is_loaded() -> bool {
    !storage::WaitSync.load(Ordering::Relaxed).is_null()
  }

  /// Load `WaitSync` using the provided loader.
  ///
  /// ## Safety
  /// As per [`load_gl_with`](crate::load_gles_with)
  pub unsafe fn load_with<F>(mut load_fn: F)
  where
    F: FnMut(*const c_char) -> *const c_void,
  {
    if let Some(p) =
      meta_loader(&mut load_fn, &[b"glWaitSync\0", b"glWaitSyncAPPLE\0"])
    {
      storage::WaitSync.store(p.as_ptr(), Ordering::SeqCst);
    };
  }
}
