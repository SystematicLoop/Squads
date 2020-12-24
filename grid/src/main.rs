pub trait Index<I, T> {
    fn get(&self, index: I) -> Option<&T>;
}

pub struct Grid2D<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid2D<T> {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::with_capacity(0),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Default + Clone> Grid2D<T> {
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }
}

impl<T> Index<(usize, usize), T> for Grid2D<T> {
    fn get(&self, index: (usize, usize)) -> Option<&T> {
        let index = index.0 + index.1 * self.width;
        self.data.get(index)
    }}

impl<T> Index<usize, T> for Grid2D<T> {
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

impl<T> Default for Grid2D<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let grid: Grid2D<i32> = Grid2D::with_size(8, 8);
    let a = grid.get(9);
    let b = grid.get((1, 1));

    assert_eq!(a.is_some(), true);
    assert_eq!(b.is_some(), true);
    assert_eq!(a, b);
}
