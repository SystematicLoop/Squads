use std::ffi::CString;

use super::shader::Shader;

pub struct Program {
    id: u32,
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Self {
        let id = unsafe { gl::CreateProgram() };

        unsafe {
            for shader in shaders {
                gl::AttachShader(id, shader.id());
            }

            gl::LinkProgram(id);

            for shader in shaders {
                gl::DetachShader(id, shader.id());
            }
        }

        let mut success = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            unsafe {
                let mut length = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut length);

                let buffer = vec![0u8; length as usize + 1];
                let error_message = std::ffi::CString::from_vec_unchecked(buffer);
                gl::GetProgramInfoLog(
                    id,
                    length,
                    std::ptr::null_mut(),
                    error_message.as_ptr() as _,
                );

                let bytes = error_message.to_bytes();
                let length = bytes.len() - 1;
                let error_message = String::from_utf8_lossy(&bytes[0..length]);
                panic!("{}", error_message)
            }
        } else {
            Self { id }
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn location(&self, name: &str) -> Option<i32> {
        let name = CString::new(name);
        if let Ok(name) = name {
            let location = unsafe { gl::GetUniformLocation(self.id, name.as_ptr() as _) };

            Some(location)
        } else {
            None
        }
    }

    pub fn uniform_1i(&mut self, name: &str, v0: i32) {
        unsafe {
            gl::Uniform1i(self.location(name).unwrap_or(-1), v0);
        }
    }

    pub fn uniform_2i(&mut self, name: &str, v0: i32, v1: i32) {
        unsafe {
            gl::Uniform2i(self.location(name).unwrap_or(-1), v0, v1);
        }
    }

    pub fn uniform_3i(&mut self, name: &str, v0: i32, v1: i32, v2: i32) {
        unsafe {
            gl::Uniform3i(self.location(name).unwrap_or(-1), v0, v1, v2);
        }
    }

    pub fn uniform_4i(&mut self, name: &str, v0: i32, v1: i32, v2: i32, v3: i32) {
        unsafe {
            gl::Uniform4i(self.location(name).unwrap_or(-1), v0, v1, v2, v3);
        }
    }

    pub fn uniform_1f(&mut self, name: &str, v0: f32) {
        unsafe {
            gl::Uniform1f(self.location(name).unwrap_or(-1), v0);
        }
    }

    pub fn uniform_2f(&mut self, name: &str, v0: f32, v1: f32) {
        unsafe {
            gl::Uniform2f(self.location(name).unwrap_or(-1), v0, v1);
        }
    }

    pub fn uniform_3f(&mut self, name: &str, v0: f32, v1: f32, v2: f32) {
        unsafe {
            gl::Uniform3f(self.location(name).unwrap_or(-1), v0, v1, v2);
        }
    }

    pub fn uniform_4f(&mut self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            gl::Uniform4f(self.location(name).unwrap_or(-1), v0, v1, v2, v3);
        }
    }

    pub fn uniform_1u(&mut self, name: &str, v0: u32) {
        unsafe {
            gl::Uniform1ui(self.location(name).unwrap_or(-1), v0);
        }
    }

    pub fn uniform_2u(&mut self, name: &str, v0: u32, v1: u32) {
        unsafe {
            gl::Uniform2ui(self.location(name).unwrap_or(-1), v0, v1);
        }
    }

    pub fn uniform_3u(&mut self, name: &str, v0: u32, v1: u32, v2: u32) {
        unsafe {
            gl::Uniform3ui(self.location(name).unwrap_or(-1), v0, v1, v2);
        }
    }

    pub fn uniform_4u(&mut self, name: &str, v0: u32, v1: u32, v2: u32, v3: u32) {
        unsafe {
            gl::Uniform4ui(self.location(name).unwrap_or(-1), v0, v1, v2, v3);
        }
    }

    pub fn uniform_array_1i(&mut self, name: &str, values: &[i32]) {
        unsafe {
            gl::Uniform1iv(
                self.location(name).unwrap_or(-1),
                values.len() as i32,
                values.as_ptr() as _,
            );
        }
    }
}
