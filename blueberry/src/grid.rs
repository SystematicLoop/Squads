#[derive(Debug)]
pub struct Grid<T: Clone> {
    columns: u32,
    rows: u32,
    data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn filled(element: T, columns: u32, rows: u32) -> Self {
        Self {
            columns,
            rows,
            data: vec![element; (columns * rows) as usize],
        }
    }

    pub fn from_slice(slice: &[T], columns: u32, rows: u32) -> Self {
        let size = columns * rows;
        assert_eq!(slice.len(), size as usize);

        Self {
            columns,
            rows,
            data: slice.to_vec(),
        }
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn size(&self) -> u32 {
        self.columns * self.rows
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    pub fn get_at(&self, x: i32, y: i32) -> Option<&T> {
        // Check if the coordinates are negative.
        if x < 0 || y < 0 {
            return None;
        }

        // Cast coordinates into unsigned spaced.
        let x = x as u32;
        let y = y as u32;

        // Check if the coordinates are in-bounds.
        if x >= self.columns || y >= self.rows {
            return None;
        }

        // Collapse coordinates into one dimension.
        let index = (x + y * self.columns) as usize;
        Some(&self.data[index])
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
