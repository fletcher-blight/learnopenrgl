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
        .window("LearnOpenGL: Framebuffers - Reverse Mirror", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    let framebuffer_shader_program = learnopenrgl::utils::Program::new(&[
        learnopenrgl::utils::Shader::new(include_str!("framebuffer.vert"), rgl::ShaderType::Vertex),
        learnopenrgl::utils::Shader::new(
            include_str!("framebuffer.frag"),
            rgl::ShaderType::Fragment,
        ),
    ]);
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    let screen_shader_program = learnopenrgl::utils::Program::new(&[
        learnopenrgl::utils::Shader::new(include_str!("screen.vert"), rgl::ShaderType::Vertex),
        learnopenrgl::utils::Shader::new(include_str!("screen.frag"), rgl::ShaderType::Fragment),
    ]);
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    {
        let projection = glm::perspective(
            window.size().0 as f32 / window.size().1 as f32,
            45.0f32.to_radians(),
            0.1,
            100.0,
        );

        framebuffer_shader_program.enable();
        framebuffer_shader_program.set_uniform_i32("texture_sampler", 0);
        framebuffer_shader_program.set_uniform_mat4_multi(
            "projection",
            rgl::MatrixOrderMajor::Column,
            *projection.as_ref(),
        );

        rgl::use_program(rgl::Program::default());
        assert_eq!(rgl::get_error(), rgl::Error::NoError);
    }

    {
        screen_shader_program.enable();
        screen_shader_program.set_uniform_i32("fragment_texture", 0);
        rgl::use_program(rgl::Program::default());
        assert_eq!(rgl::get_error(), rgl::Error::NoError);
    }

    let cube_vao = load_vertex_array(get_cube_data());
    let plane_vao = load_vertex_array(get_plane_data());
    let quad_vao = load_quad_vertex_array(get_quad_data());
    let mirror_vao = load_quad_vertex_array(get_mirror_data());
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    println!("Loading asset: metal.png ...");
    let texture_metal = load_texture(
        image::load_from_memory(include_bytes!("../../../assets/metal.png"))?,
        rgl::TextureWrapMode::Repeat,
    )?;
    println!("Loading asset: metal.png ... done");
    println!("Loading asset: container.jpg ...");
    let texture_container = load_texture(
        image::load_from_memory(include_bytes!("../../../assets/container.jpg"))?,
        rgl::TextureWrapMode::Repeat,
    )?;
    println!("Loading asset: container.jpg ... done");
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    let mut fbo = rgl::Framebuffer::default();
    rgl::gen_framebuffers(std::slice::from_mut(&mut fbo));
    rgl::bind_framebuffer(rgl::FramebufferBindingTarget::ReadDraw, fbo);

    let mut texture_buffer = rgl::Texture::default();
    rgl::gen_textures(std::slice::from_mut(&mut texture_buffer));
    rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_buffer);
    rgl::tex_image_2d(
        rgl::TextureBinding2DTarget::Image2D,
        0,
        rgl::TextureInternalFormat::RGB,
        window.size().0,
        window.size().1,
        rgl::TextureFormat::RGB,
        rgl::TexturePixelType::U8,
        rgl::TextureData::<()>::Reserve,
    );
    rgl::texture_target_min_filter(
        rgl::TextureBindingTarget::Image2D,
        rgl::TextureMinFilter::Linear,
    );
    rgl::texture_target_mag_filter(
        rgl::TextureBindingTarget::Image2D,
        rgl::TextureMagFilter::Linear,
    );
    rgl::framebuffer_texture_2d(
        rgl::FramebufferBindingTarget::ReadDraw,
        rgl::FramebufferAttachment::Colour(0),
        rgl::TextureBinding2DTarget::Image2D,
        texture_buffer,
        0,
    );

    let mut rbo = rgl::Renderbuffer::default();
    rgl::gen_renderbuffers(std::slice::from_mut(&mut rbo));
    rgl::bind_renderbuffer(rbo);
    rgl::renderbuffer_storage(
        rgl::RenderbufferInternalFormat::Depth24Stencil8,
        window.size().0,
        window.size().1,
    );
    rgl::framebuffer_renderbuffer(
        rgl::FramebufferBindingTarget::ReadDraw,
        rgl::FramebufferAttachment::DepthStencil,
        rbo,
    );

    assert_eq!(
        rgl::check_framebuffer_status(rgl::FramebufferBindingTarget::ReadDraw),
        Some(rgl::FramebufferStatus::Complete)
    );
    rgl::bind_framebuffer(
        rgl::FramebufferBindingTarget::ReadDraw,
        rgl::Framebuffer::default(),
    );

    // ============================================================================================

    let mut inception = false;
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
                    keycode: Some(sdl2::keyboard::Keycode::I),
                    ..
                } => {
                    inception = !inception;
                }
                event => learnopenrgl::utils::process_sdl_event(&mut camera, event),
            }
        }
        camera.update_position(frame.last_frame_duration(), 3.0);

        {
            framebuffer_shader_program.enable();
            framebuffer_shader_program.set_uniform_mat4_multi(
                "view",
                rgl::MatrixOrderMajor::Column,
                camera.calculate_view(),
            );

            rgl::bind_framebuffer(rgl::FramebufferBindingTarget::ReadDraw, fbo);
            rgl::enable(rgl::Capability::DepthTest);
            rgl::clear_colour(0.1, 0.1, 0.1, 0.1);
            rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);
            assert_eq!(rgl::get_error(), rgl::Error::NoError);

            {
                rgl::bind_vertex_array(plane_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_metal);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 6);
                rgl::bind_vertex_array(rgl::VertexArray::default());
                assert_eq!(rgl::get_error(), rgl::Error::NoError);
            }

            {
                rgl::bind_vertex_array(cube_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_container);
                draw_container_cubes(&framebuffer_shader_program);
                rgl::bind_vertex_array(rgl::VertexArray::default());
                assert_eq!(rgl::get_error(), rgl::Error::NoError);
            }

            rgl::bind_framebuffer(
                rgl::FramebufferBindingTarget::ReadDraw,
                rgl::Framebuffer::default(),
            );
            rgl::clear_colour(0.1, 0.1, 0.1, 0.1);
            rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

            screen_shader_program.enable();
            rgl::disable(rgl::Capability::DepthTest);
            assert_eq!(rgl::get_error(), rgl::Error::NoError);

            {
                rgl::bind_vertex_array(quad_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_buffer);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 6);
                rgl::bind_vertex_array(rgl::VertexArray::default());
                assert_eq!(rgl::get_error(), rgl::Error::NoError);
            }

            if inception {
                framebuffer_shader_program.enable();
                rgl::enable(rgl::Capability::DepthTest);
                rgl::bind_vertex_array(cube_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_buffer);
                draw_container_cubes(&framebuffer_shader_program);
                rgl::bind_vertex_array(rgl::VertexArray::default());
                assert_eq!(rgl::get_error(), rgl::Error::NoError);
            }
        }
        {
            let (yaw, pitch) = camera.get_yaw_pitch();
            camera.set_yaw_pitch(yaw + 180.0, -pitch);

            framebuffer_shader_program.enable();
            framebuffer_shader_program.set_uniform_mat4_multi(
                "view",
                rgl::MatrixOrderMajor::Column,
                camera.calculate_view(),
            );

            rgl::bind_framebuffer(rgl::FramebufferBindingTarget::ReadDraw, fbo);
            rgl::enable(rgl::Capability::DepthTest);
            rgl::clear_colour(0.1, 0.1, 0.1, 0.1);
            rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);
            assert_eq!(rgl::get_error(), rgl::Error::NoError);

            {
                rgl::bind_vertex_array(plane_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_metal);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 6);
                rgl::bind_vertex_array(rgl::VertexArray::default());
                assert_eq!(rgl::get_error(), rgl::Error::NoError);
            }

            {
                rgl::bind_vertex_array(cube_vao);
                rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_container);
                draw_container_cubes(&framebuffer_shader_program);
                rgl::bind_vertex_array(rgl::VertexArray::default());
                assert_eq!(rgl::get_error(), rgl::Error::NoError);
            }

            rgl::bind_framebuffer(
                rgl::FramebufferBindingTarget::ReadDraw,
                rgl::Framebuffer::default(),
            );
            rgl::enable(rgl::Capability::DepthTest);

            screen_shader_program.enable();
            rgl::bind_vertex_array(mirror_vao);
            rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture_buffer);
            rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 6);
            rgl::bind_vertex_array(rgl::VertexArray::default());
            assert_eq!(rgl::get_error(), rgl::Error::NoError);

            camera.set_yaw_pitch(yaw, pitch);
        }

        window.gl_swap_window();
        assert_eq!(rgl::get_error(), rgl::Error::NoError);
    }

    Ok(())
}

fn draw_container_cubes(program: &learnopenrgl::utils::Program) {
    program.set_uniform_mat4_multi(
        "model",
        rgl::MatrixOrderMajor::Column,
        *glm::translate(&glm::one(), &glm::vec3(-1.0, 0.0, -1.0)).as_ref(),
    );
    rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);

    program.set_uniform_mat4_multi(
        "model",
        rgl::MatrixOrderMajor::Column,
        *glm::translate(&glm::one(), &glm::vec3(2.0, 0.0, 0.0)).as_ref(),
    );
    rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
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

fn load_quad_vertex_array(data: &[f32]) -> rgl::VertexArray {
    let mut vao = rgl::VertexArray::default();
    rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));

    let mut vbo = rgl::Buffer::default();
    rgl::gen_buffers(std::slice::from_mut(&mut vbo));

    rgl::bind_vertex_array(vao);
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, vbo);

    let stride = 4 * std::mem::size_of::<f32>() as u64;
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        data,
        rgl::BufferUsageFrequency::Static,
        rgl::BufferUsageNature::Draw,
    );
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_float_pointer(
        0,
        rgl::VertexAttribSize::Double,
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
        2 * std::mem::size_of::<f32>() as u64,
    );

    rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer::default());
    rgl::bind_vertex_array(rgl::VertexArray::default());

    vao
}

fn load_texture(
    image: image::DynamicImage,
    wrap_mode: rgl::TextureWrapMode,
) -> anyhow::Result<rgl::Texture> {
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
    rgl::texture_target_wrap_s(rgl::TextureBindingTarget::Image2D, wrap_mode);
    rgl::texture_target_wrap_t(rgl::TextureBindingTarget::Image2D, wrap_mode);
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

fn get_quad_data() -> &'static [f32] {
    #[rustfmt::skip]
    let data = &[
        // position         // texture Coord
        -1.0, -1.0,         0.0, 0.0,
        -1.0, 1.0,          0.0, 1.0,
        1.0, 1.0,           1.0, 1.0,
        
        -1.0, -1.0,         0.0, 0.0,
        1.0, 1.0,           1.0, 1.0,
        1.0, -1.0,          1.0, 0.0,
    ];

    data
}

fn get_mirror_data() -> &'static [f32] {
    #[rustfmt::skip]
        let data = &[
        // position         // texture Coord
        -0.4, 0.6,         0.0, 0.0,
        -0.4, 1.0,          0.0, 1.0,
        0.4, 1.0,           1.0, 1.0,

        -0.4, 0.6,         0.0, 0.0,
        0.4, 1.0,           1.0, 1.0,
        0.4, 0.6,          1.0, 0.0,
    ];

    data
}
