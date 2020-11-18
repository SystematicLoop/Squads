pub struct IndexBuffer {
    id: u32,
    length: usize,
    capacity: usize,
}

impl IndexBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(gl::ARRAY_BUFFER, capacity as _, 0 as _, gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self {
            id,
            length: 0,
            capacity,
        }
    }

    pub fn set_indices(&mut self, indices: &[u32]) {
        unsafe {
            let size = indices.len() * std::mem::size_of::<u32>();

            assert!(
                size <= self.capacity,
                "Attempt to update index buffer with data size {} when its capacity is {}.",
                size,
                self.capacity
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                size as _,
                indices.as_ptr() as _,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            self.length = indices.len();
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
