use nalgebra_glm as glm;
use rgl::prelude as rgl;

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    sdl.mouse().set_relative_mouse_mode(true);
    let window = video
        .window("LearnOpenGL: Depth Testing - Depth Func", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    let shader_program = {
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

        rgl::detach_shader(shader_program, vertex_shader);
        rgl::detach_shader(shader_program, fragment_shader);
        rgl::delete_shader(vertex_shader);
        rgl::delete_shader(fragment_shader);

        shader_program
    };
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    let cube_vao = load_vertex_array(get_cube_data());
    let plane_vao = load_vertex_array(get_plane_data());
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    let (model_location, view_location) = {
        rgl::use_program(shader_program);

        let loc_model = rgl::get_uniform_location(
            shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
        );
        let loc_view = rgl::get_uniform_location(
            shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
        );
        let loc_projection = rgl::get_uniform_location(
            shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
        );
        let loc_texture_sampler = rgl::get_uniform_location(
            shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"texture_sampler\0")?,
        );

        let data_projection = glm::perspective(
            window.size().0 as f32 / window.size().1 as f32,
            45.0f32.to_radians(),
            0.1,
            100.0,
        );

        rgl::uniform_matrix_4f32v_multi(
            loc_projection,
            rgl::MatrixOrderMajor::Column,
            &[*data_projection.as_ref()],
        );
        rgl::uniform_1i32(loc_texture_sampler, 0);

        rgl::use_program(rgl::Program::default());

        (loc_model, loc_view)
    };
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    println!("Loading asset: metal.png ...");
    let texture_metal = load_texture(image::load_from_memory(include_bytes!(
        "../../../assets/metal.png"
    ))?)?;
    println!("Loading asset: metal.png ... done");
    println!("Loading asset: marble.jpg ...");
    let texture_marble = load_texture(image::load_from_memory(include_bytes!(
        "../../../assets/marble.jpg"
    ))?)?;
    println!("Loading asset: marble.jpg ... done");
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    rgl::enable(rgl::Capability::DepthTest);
    rgl::clear_colour(0.1, 0.1, 0.1, 0.1);
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    // ============================================================================================

    let mut camera = learnopenrgl::utils::Camera::new();
    let mut frame = learnopenrgl::utils::Frame::new_now();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        let frame = frame.mark_new_frame();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Num1),
                    ..
                } => rgl::depth_func(rgl::CompareFunc::Less),
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Num2),
                    ..
                } => rgl::depth_func(rgl::CompareFunc::Always),
                event => learnopenrgl::utils::process_sdl_event(&mut camera, event),
            }
        }
        camera.update_position(frame.last_frame_duration(), 3.0);

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

        {
            rgl::use_program(shader_program);
            rgl::active_texture(0);

            rgl::uniform_matrix_4f32v_multi(
                view_location,
                rgl::MatrixOrderMajor::Column,
                &[camera.calculate_view()],
            );

            {
                rgl::bind_vertex_array(cube_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_marble);

                rgl::uniform_matrix_4f32v_multi(
                    model_location,
                    rgl::MatrixOrderMajor::Column,
                    &[*glm::translate(&glm::one(), &glm::vec3(-1.0, 0.0, -1.0)).as_ref()],
                );
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);

                rgl::uniform_matrix_4f32v_multi(
                    model_location,
                    rgl::MatrixOrderMajor::Column,
                    &[*glm::translate(&glm::one(), &glm::vec3(2.0, 0.0, 0.0)).as_ref()],
                );
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);

                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, rgl::Texture::default());
                rgl::bind_vertex_array(rgl::VertexArray::default());
            }

            {
                rgl::bind_vertex_array(plane_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_metal);

                rgl::uniform_matrix_4f32v_multi(
                    model_location,
                    rgl::MatrixOrderMajor::Column,
                    &[*glm::Mat4::identity().as_ref()],
                );
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 6);

                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, rgl::Texture::default());
                rgl::bind_vertex_array(rgl::VertexArray::default());
            }

            rgl::use_program(rgl::Program::default());
        }

        assert_eq!(rgl::get_error(), rgl::Error::NoError);
        window.gl_swap_window();
    }

    Ok(())
}

fn load_vertex_array(data: &[f32]) -> rgl::VertexArray {
    let mut vao = rgl::VertexArray::default();
    rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));

    let mut vbo = rgl::Buffer::default();
    rgl::gen_buffers(std::slice::from_mut(&mut vbo));

    rgl::bind_vertex_array(vao);
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, vbo);

    let stride = 5 * std::mem::size_of::<f32>() as u64;
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        data,
        rgl::BufferUsageFrequency::Static,
        rgl::BufferUsageNature::Draw,
    );
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_float_pointer(
        0,
        rgl::VertexAttribSize::Triple,
        rgl::VertexAttribFloatType::F32,
        false,
        stride,
        0,
    );
    rgl::enable_vertex_attrib_array(1);
    rgl::vertex_attrib_float_pointer(
        1,
        rgl::VertexAttribSize::Double,
        rgl::VertexAttribFloatType::F32,
        false,
        stride,
        3 * std::mem::size_of::<f32>() as u64,
    );

    rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer::default());
    rgl::bind_vertex_array(rgl::VertexArray::default());

    vao
}

fn load_texture(image: image::DynamicImage) -> anyhow::Result<rgl::Texture> {
    let format = match image.color().channel_count() {
        1 => Ok(rgl::TextureFormat::R),
        3 => Ok(rgl::TextureFormat::RGB),
        4 => Ok(rgl::TextureFormat::RGBA),
        channel_count => Err(anyhow::Error::msg(format!(
            "Unsupported number of colour channels ({channel_count}) in {image:?}"
        ))),
    }?;

    let mut texture = rgl::Texture::default();
    rgl::gen_textures(std::slice::from_mut(&mut texture));
    rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture);
    rgl::tex_image_2d(
        rgl::TextureBinding2DTarget::Image2D,
        0,
        rgl::TextureInternalFormat::RGBA,
        image.width(),
        image.height(),
        format,
        rgl::TexturePixelType::U8,
        rgl::TextureData::Data(image.as_bytes()),
    );
    rgl::generate_mipmap(rgl::TextureBindingTarget::Image2D);
    rgl::texture_target_wrap_s(
        rgl::TextureBindingTarget::Image2D,
        rgl::TextureWrapMode::Repeat,
    );
    rgl::texture_target_wrap_t(
        rgl::TextureBindingTarget::Image2D,
        rgl::TextureWrapMode::Repeat,
    );
    rgl::texture_target_min_filter(
        rgl::TextureBindingTarget::Image2D,
        rgl::TextureMinFilter::LinearMipmapLinear,
    );
    rgl::texture_target_mag_filter(
        rgl::TextureBindingTarget::Image2D,
        rgl::TextureMagFilter::Linear,
    );

    rgl::bind_texture(rgl::TextureBindingTarget::Image2D, rgl::Texture::default());
    Ok(texture)
}

fn get_cube_data() -> &'static [f32] {
    #[rustfmt::skip]
        let data = &[
        // positions        // texture Coord
        -0.5, -0.5, -0.5,   0.0, 0.0,
        0.5, -0.5, -0.5,    1.0, 0.0,
        0.5, 0.5, -0.5,     1.0, 1.0,
        0.5, 0.5, -0.5,     1.0, 1.0,
        -0.5, 0.5, -0.5,    0.0, 1.0,
        -0.5, -0.5, -0.5,   0.0, 0.0,

        -0.5, -0.5, 0.5,    0.0, 0.0,
        0.5, -0.5, 0.5,     1.0, 0.0,
        0.5, 0.5, 0.5,      1.0, 1.0,
        0.5, 0.5, 0.5,      1.0, 1.0,
        -0.5, 0.5, 0.5,     0.0, 1.0,
        -0.5, -0.5, 0.5,    0.0, 0.0,

        -0.5, 0.5, 0.5,     1.0, 0.0,
        -0.5, 0.5, -0.5,    1.0, 1.0,
        -0.5, -0.5, -0.5,   0.0, 1.0,
        -0.5, -0.5, -0.5,   0.0, 1.0,
        -0.5, -0.5, 0.5,    0.0, 0.0,
        -0.5, 0.5, 0.5,     1.0, 0.0,

        0.5, 0.5, 0.5,      1.0, 0.0,
        0.5, 0.5, -0.5,     1.0, 1.0,
        0.5, -0.5, -0.5,    0.0, 1.0,
        0.5, -0.5, -0.5,    0.0, 1.0,
        0.5, -0.5, 0.5,     0.0, 0.0,
        0.5, 0.5, 0.5,      1.0, 0.0,

        -0.5, -0.5, -0.5,   0.0, 1.0,
        0.5, -0.5, -0.5,    1.0, 1.0,
        0.5, -0.5, 0.5,     1.0, 0.0,
        0.5, -0.5, 0.5,     1.0, 0.0,
        -0.5, -0.5, 0.5,    0.0, 0.0,
        -0.5, -0.5, -0.5,   0.0, 1.0,

        -0.5, 0.5, -0.5,    0.0, 1.0,
        0.5, 0.5, -0.5,     1.0, 1.0,
        0.5, 0.5, 0.5,      1.0, 0.0,
        0.5, 0.5, 0.5,      1.0, 0.0,
        -0.5, 0.5, 0.5,     0.0, 0.0,
        -0.5, 0.5, -0.5,    0.0, 1.0,
    ];

    data
}

fn get_plane_data() -> &'static [f32] {
    // note the texture Coords are set higher than 1, together with TextureWrapMode::Repeat. This
    // will cause the floor texture to repeat

    #[rustfmt::skip]
        let data = &[
        // position         // texture Coord
        5.0, -0.5,  5.0,    2.0, 0.0,
        -5.0, -0.5,  5.0,   0.0, 0.0,
        -5.0, -0.5, -5.0,   0.0, 2.0,

        5.0, -0.5,  5.0,    2.0, 0.0,
        -5.0, -0.5, -5.0,   0.0, 2.0,
        5.0, -0.5, -5.0,    2.0, 2.0,
    ];

    data
}
