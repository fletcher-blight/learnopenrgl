use rgl::prelude as rgl;

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    let window = video
        .window("LearnOpenGL: Hello Triangle", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex);
    rgl::shader_source(vertex_shader, include_str!("shader.vert"));
    rgl::compile_shader(vertex_shader);
    assert!(rgl::get_shader_compile_status(vertex_shader));

    let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment);
    rgl::shader_source(fragment_shader, include_str!("shader.frag"));
    rgl::compile_shader(fragment_shader);
    assert!(rgl::get_shader_compile_status(fragment_shader));

    let shader_program = rgl::create_program();
    rgl::attach_shader(shader_program, vertex_shader);
    rgl::attach_shader(shader_program, fragment_shader);
    rgl::link_program(shader_program);
    assert!(rgl::get_program_link_status(shader_program));

    let mut vao = Default::default();
    rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));

    let mut vbo = Default::default();
    rgl::gen_buffers(std::slice::from_mut(&mut vbo));

    rgl::bind_vertex_array(vao);
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, vbo);
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        &[[-0.5, -0.5], [0.0, 0.5], [0.5, -0.5f32]],
        rgl::BufferUsageFrequency::Static,
        rgl::BufferUsageNature::Draw,
    );
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_float_pointer(
        0,
        rgl::VertexAttribSize::Double,
        rgl::VertexAttribFloatType::F32,
        false,
        (std::mem::size_of::<f32>() * 2) as u64,
        0,
    );
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer(0));

    // ============================================================================================

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);
        rgl::use_program(shader_program);
        rgl::bind_vertex_array(vao);
        rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 3);

        assert_eq!(rgl::get_error(), rgl::Error::NoError);
        window.gl_swap_window();
    }

    Ok(())
}
