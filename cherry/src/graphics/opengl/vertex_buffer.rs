use super::layout::Layout;

pub struct VertexBuffer {
    id: u32,
    capacity: usize,
    layout: Layout,
}

impl VertexBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(gl::ARRAY_BUFFER, capacity as _, 0 as _, gl::DYNAMIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        let layout = Layout::default();

        Self {
            id,
            capacity,
            layout,
        }
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }

    pub fn set_data<T: Sized>(&mut self, data: &[T]) {
        unsafe {
            let data = as_u8_slice(data);
            let size = data.len();

            assert!(
                size <= self.capacity,
                "Attempt to update vertex buffer with data size {} when its capacity is {}.",
                size,
                self.capacity
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, size as _, data.as_ptr() as _);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

fn as_u8_slice<T: Sized>(slice: &[T]) -> &[u8] {
    unsafe {
        let bytes = slice.as_ptr() as *const u8;
        let len = std::mem::size_of::<T>() * slice.len();

        std::slice::from_raw_parts(bytes, len)
    }
}
