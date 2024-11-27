use super::VertexArrayId;
use glow::HasContext;

/// Sets up a simple VAO/VBO to render a single quad
/// Used for presenting the backing buffer and in post-process chains.
pub fn setup_quad(gl: &glow::Context) -> VertexArrayId {
    #[rustfmt::skip]
    let quad_vertices: [f32; 24] = [
        // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
        // positions // texCoords
        -1.0,  1.0, 0.0, 1.0,
        -1.0, -1.0, 0.0, 0.0,
         1.0, -1.0, 1.0, 0.0,
        -1.0,  1.0, 0.0, 1.0,
         1.0, -1.0, 1.0, 0.0,
         1.0,  1.0, 1.0, 1.0,
    ];
    let (vertex_array, vertex_buffer);
    unsafe {
        vertex_array = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vertex_array));

        vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            quad_vertices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );

        gl.enable_vertex_attrib_array(0);
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
        let stride = 4 * std::mem::size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            stride,
            2 * std::mem::size_of::<f32>() as i32,
        );

        gl.bind_vertex_array(None);
    }

    vertex_array
}

/// Sets up a simple VAO/VBO to render a single quad
/// Used for presenting the backing buffer and in post-process chains.
pub fn setup_quad_gutter(
    gl: &glow::Context,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
) -> VertexArrayId {
    #[rustfmt::skip]
    let quad_vertices: [f32; 24] = [
        // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
        // positions // texCoords
        left,  top, 0.0, 1.0,
        left, bottom, 0.0, 0.0,
        right, bottom, 1.0, 0.0,
        left,  top, 0.0, 1.0,
        right, bottom, 1.0, 0.0,
        right, top, 1.0, 1.0,
    ];
    let (vertex_array, vertex_buffer);
    unsafe {
        vertex_array = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vertex_array));

        vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            quad_vertices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );

        gl.enable_vertex_attrib_array(0);
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
        let stride = 4 * std::mem::size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            stride,
            2 * std::mem::size_of::<f32>() as i32,
        );

        gl.bind_vertex_array(None);
    }

    vertex_array
}
