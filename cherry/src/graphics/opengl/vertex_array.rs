use super::{
    index_buffer::IndexBuffer,
    vertex_buffer::VertexBuffer,
};

pub struct VertexArray {
    id: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }

        Self { id }
    }

    pub fn set_vertex_buffer(&mut self, buffer: &VertexBuffer) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer.id());
        }

        let layout = buffer.layout();
        for element in &layout.elements {
            unsafe {
                gl::EnableVertexAttribArray(element.index as u32);
                gl::VertexAttribPointer(
                    element.index as _,
                    element.components as _,
                    element.data_type,
                    gl::FALSE,
                    layout.size as _,
                    element.offset as _,
                );
            }
        }

        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set_index_buffer(&mut self, buffer: &IndexBuffer) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id());
            gl::BindVertexArray(0);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
