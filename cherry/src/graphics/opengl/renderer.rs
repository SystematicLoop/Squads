use blueberry::{
    Vec2f,
    Vec3f,
};

use crate::{
    graphics::font::Font,
    terminal::{
        buffer::Buffer,
        tile::Tile,
    },
};

use super::{
    index_buffer::IndexBuffer,
    layout::{
        ElementKind,
        Layout,
    },
    program::Program,
    shader::{
        Shader,
        ShaderKind,
    },
    vertex_array::VertexArray,
    vertex_buffer::VertexBuffer,
};

pub struct Renderer {
    shader: Program,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    vertex_array: VertexArray,
    vertices: Vec<Vertex>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    position: Vec2f,
    background: Vec3f,
    foreground: Vec3f,
    texture_coords: Vec2f,
}

impl Renderer {
    pub fn new(columns: u32, rows: u32) -> Self {
        // Initialise the layout for the vertex buffer.
        let layout = Layout::builder()
            .with(ElementKind::Float2) // Position
            .with(ElementKind::Float3) // Foreground
            .with(ElementKind::Float3) // Background
            .with(ElementKind::Float2) // Texture Coords
            .build();

        // Calculate the maximum number of vertices and indices.
        let cell_count = (columns * rows) as usize;
        let vertice_count = cell_count * 4;
        let indice_count = cell_count * 6;

        // Initialise vertice and indice data.
        let vertices = Vec::with_capacity(vertice_count);
        let mut indices = Vec::with_capacity(indice_count);

        // Generate indices. Since we are rendering quads, these are
        // a one-time calculation.
        let mut offset = 0;
        for _ in 0..cell_count {
            indices.push(offset + 0);
            indices.push(offset + 1);
            indices.push(offset + 2);
            indices.push(offset + 2);
            indices.push(offset + 3);
            indices.push(offset + 0);

            offset += 4;
        }

        let vertex_buffer_size = vertice_count * std::mem::size_of::<Vertex>();
        let index_buffer_size = indice_count * std::mem::size_of::<u32>();

        let mut vertex_buffer = VertexBuffer::new(vertex_buffer_size);
        vertex_buffer.set_layout(layout);
        vertex_buffer.set_data(&vertices);

        let mut index_buffer = IndexBuffer::new(index_buffer_size);
        index_buffer.set_indices(&indices);

        let mut vertex_array = VertexArray::new();
        vertex_array.set_vertex_buffer(&vertex_buffer);
        vertex_array.set_index_buffer(&index_buffer);

        let vertex_shader_source = r#"#version 450 core
        layout (location = 0) in vec2 a_vertex_position;
        layout (location = 1) in vec3 a_foreground;
        layout (location = 2) in vec3 a_background;
        layout (location = 3) in vec2 a_texture_coords;
        
        out vec3 foreground;
        out vec3 background;
        out vec2 texture_coords;
        
        uniform vec2 font_size;
        uniform vec2 view_size;
        
        void main() {
            texture_coords = a_texture_coords;
            foreground = a_foreground;
            background = a_background;
        
            vec2 vertex_position = ((a_vertex_position * font_size) / (view_size / 2.0)) - vec2(1.0, 1.0);
        
            gl_Position = vec4(vertex_position, 0.0, 1.0);
        }"#;

        let fragment_shader_source = r#"#version 450 core
        uniform sampler2D u_texture;
        
        in vec3 foreground;
        in vec3 background;
        in vec2 texture_coords;
        
        out vec4 fragment_colour;
        
        void main() {
            vec3 sample_colour = texture(u_texture, texture_coords).xyz;
            vec3 foreground_mask = (vec3(1.0) - sample_colour) * foreground;
            vec3 background_mask = sample_colour * background;
            vec3 colour = foreground_mask + background_mask;
        
            fragment_colour = vec4(colour, 1.0);
        }"#;

        let shader = Program::new(&[
            Shader::new(ShaderKind::Vertex, vertex_shader_source),
            Shader::new(ShaderKind::Fragment, fragment_shader_source),
        ]);

        Self {
            shader,
            vertex_buffer,
            index_buffer,
            vertex_array,
            vertices,
        }
    }

    fn draw_tile(&mut self, font: &Font, position: Vec2f, tile: &Tile) {
        // Calculate vertex positions.
        let positions = [
            position + Vec2f::new(0.0, 0.0),
            position + Vec2f::new(1.0, 0.0),
            position + Vec2f::new(1.0, 1.0),
            position + Vec2f::new(0.0, 1.0),
        ];

        // Normalise colours.
        let foreground = Vec3f::new(
            tile.fg.r as f32 / 255.0,
            tile.fg.g as f32 / 255.0,
            tile.fg.b as f32 / 255.0,
        );

        let background = Vec3f::new(
            tile.bg.r as f32 / 255.0,
            tile.bg.g as f32 / 255.0,
            tile.bg.b as f32 / 255.0,
        );

        // Column and row of glyph into font sheet.
        let column = tile.glyph as u32 % 16;
        let row = tile.glyph as u32 / 16;

        // The bottom-left coordinates of the glyph.
        let x = column * font.width();
        let y = row * font.height();

        // Calculate sub-texture coordinates of the glyph.
        let u0 = x as f32 / font.texture().width() as f32;
        let v0 = y as f32 / font.texture().height() as f32;
        let u1 = (x + font.width()) as f32 / font.texture().width() as f32;
        let v1 = (y + font.height()) as f32 / font.texture().height() as f32;

        let texture_coords = [
            Vec2f::new(u0, v1),
            Vec2f::new(u1, v1),
            Vec2f::new(u1, v0),
            Vec2f::new(u0, v0),
        ];

        for i in 0..4 {
            let vertex = Vertex {
                position: positions[i],
                foreground,
                background,
                texture_coords: texture_coords[i],
            };

            self.vertices.push(vertex);
        }
    }

    pub fn draw_buffer(&mut self, font: &Font, offset: Vec2f, buffer: &mut Buffer) {
        for r in 0..buffer.rows() {
            for c in 0..buffer.columns() {
                let position = offset + Vec2f::new(c as f32, (buffer.rows() - r - 1) as f32);
                let tile = &mut buffer.get_at(c as i32, r as i32).unwrap();

                self.draw_tile(font, position, tile);
            }
        }

        self.vertex_buffer.set_data(&self.vertices);

        unsafe {
            gl::UseProgram(self.shader.id());
            gl::BindTextureUnit(0, font.texture().id());
            gl::BindVertexArray(self.vertex_array.id());

            self.shader
                .uniform_2f("font_size", font.width() as f32, font.height() as f32);
            self.shader.uniform_2f(
                "view_size",
                (font.width() * buffer.columns()) as f32,
                (font.height() * buffer.rows()) as f32,
            );
            self.shader
                .uniform_1i("texture", font.texture().id() as i32);

            gl::DrawElements(
                gl::TRIANGLES,
                self.index_buffer.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
            gl::BindTextureUnit(0, 0);
            gl::UseProgram(0);
        }

        self.vertices.clear();
    }
}
