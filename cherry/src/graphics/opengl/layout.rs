#[derive(Debug, Default)]
pub struct Layout {
    pub elements: Vec<Element>,
    pub size: usize,
}

impl Layout {
    pub fn builder() -> Builder {
        Builder {
            elements: Vec::new(),
            size: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Element {
    pub index: usize,
    pub components: u32,
    pub size: usize,
    pub offset: usize,
    pub data_type: u32,
}

pub struct Builder {
    elements: Vec<Element>,
    size: usize,
}

impl Builder {
    pub fn with(mut self, kind: ElementKind) -> Self {
        let element_index = self.elements.len();
        let element_components = kind.gl_components();
        let element_size = kind.gl_size();
        let element_offset = self.size;
        let element_type = kind.gl_type();

        let element = Element {
            index: element_index,
            components: element_components,
            size: element_size,
            offset: element_offset,
            data_type: element_type,
        };

        self.size += element_size;
        self.elements.push(element);

        self
    }

    pub fn build(self) -> Layout {
        Layout {
            elements: self.elements,
            size: self.size,
        }
    }
}

#[derive(Debug)]
pub enum ElementKind {
    Int,
    Float,
    Float2,
    Float3,
    Float4,
}

impl ElementKind {
    fn gl_components(&self) -> u32 {
        match self {
            ElementKind::Int => 1,
            ElementKind::Float => 1,
            ElementKind::Float2 => 2,
            ElementKind::Float3 => 3,
            ElementKind::Float4 => 4,
        }
    }

    fn gl_size(&self) -> usize {
        match self {
            ElementKind::Int => 4,
            ElementKind::Float => 4,
            ElementKind::Float2 => 8,
            ElementKind::Float3 => 12,
            ElementKind::Float4 => 16,
        }
    }

    fn gl_type(&self) -> u32 {
        match self {
            ElementKind::Int => gl::INT,
            ElementKind::Float => gl::FLOAT,
            ElementKind::Float2 => gl::FLOAT,
            ElementKind::Float3 => gl::FLOAT,
            ElementKind::Float4 => gl::FLOAT,
        }
    }
}
