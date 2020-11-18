use std::ffi::CString;

#[derive(Debug)]
pub struct Shader {
    id: u32,
}

#[derive(Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment,
}

impl ShaderKind {
    fn gl_type(&self) -> u32 {
        match self {
            ShaderKind::Vertex => gl::VERTEX_SHADER,
            ShaderKind::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

impl Shader {
    pub fn new(kind: ShaderKind, source_code: &str) -> Self {
        let source = CString::new(source_code).unwrap();

        let id = unsafe { gl::CreateShader(kind.gl_type()) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            unsafe {
                let mut length = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length);

                let buffer = vec![0u8; length as usize + 1];
                let error_message = CString::from_vec_unchecked(buffer);
                gl::GetShaderInfoLog(
                    id,
                    length,
                    std::ptr::null_mut(),
                    error_message.as_ptr() as _,
                );

                let bytes = error_message.to_bytes();
                let length = bytes.len() - 1;
                let error_message = String::from_utf8_lossy(&bytes[0..length]);
                panic!("{:?}: {}", kind, error_message)
            }
        } else {
            Self { id }
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
