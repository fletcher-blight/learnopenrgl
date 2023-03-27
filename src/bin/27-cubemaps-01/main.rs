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
        .window("LearnOpenGL: CubeMaps", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    rgl::enable(rgl::Capability::DepthTest);

    let mut skybox_texture = rgl::Texture::default();
    rgl::gen_textures(std::slice::from_mut(&mut skybox_texture));
    rgl::bind_texture(rgl::TextureBindingTarget::CubeMap, skybox_texture);

    println!("Loading skybox ...");
    for (target, bytes) in [
        (
            rgl::TextureBinding2DTarget::CubeMapPositiveX,
            include_bytes!("../../../assets/skybox/right.jpg").as_slice(),
        ),
        (
            rgl::TextureBinding2DTarget::CubeMapNegativeX,
            include_bytes!("../../../assets/skybox/left.jpg").as_slice(),
        ),
        (
            rgl::TextureBinding2DTarget::CubeMapPositiveY,
            include_bytes!("../../../assets/skybox/top.jpg").as_slice(),
        ),
        (
            rgl::TextureBinding2DTarget::CubeMapNegativeY,
            include_bytes!("../../../assets/skybox/bottom.jpg").as_slice(),
        ),
        (
            rgl::TextureBinding2DTarget::CubeMapPositiveZ,
            include_bytes!("../../../assets/skybox/front.jpg").as_slice(),
        ),
        (
            rgl::TextureBinding2DTarget::CubeMapNegativeZ,
            include_bytes!("../../../assets/skybox/back.jpg").as_slice(),
        ),
    ] {
        let image = image::load_from_memory(bytes)?;
        rgl::tex_image_2d(
            target,
            0,
            rgl::TextureInternalFormat::RGB,
            image.width(),
            image.height(),
            rgl::TextureFormat::RGB,
            rgl::TexturePixelType::U8,
            rgl::TextureData::Data(image.as_bytes()),
        );
    }
    println!("Loading skybox ... done");

    rgl::texture_target_mag_filter(
        rgl::TextureBindingTarget::CubeMap,
        rgl::TextureMagFilter::Linear,
    );
    rgl::texture_target_min_filter(
        rgl::TextureBindingTarget::CubeMap,
        rgl::TextureMinFilter::Linear,
    );
    rgl::texture_target_wrap_s(
        rgl::TextureBindingTarget::CubeMap,
        rgl::TextureWrapMode::ClampToEdge,
    );
    rgl::texture_target_wrap_t(
        rgl::TextureBindingTarget::CubeMap,
        rgl::TextureWrapMode::ClampToEdge,
    );
    rgl::texture_target_wrap_r(
        rgl::TextureBindingTarget::CubeMap,
        rgl::TextureWrapMode::ClampToEdge,
    );
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    println!("Loading container ...");
    let cube_texture = load_texture(
        image::load_from_memory(include_bytes!("../../../assets/container.jpg"))?,
        rgl::TextureWrapMode::Repeat,
    )?;
    println!("Loading container ... done");

    let skybox_shader = learnopenrgl::utils::Program::new(&[
        learnopenrgl::utils::Shader::new(include_str!("skybox.vert"), rgl::ShaderType::Vertex),
        learnopenrgl::utils::Shader::new(include_str!("skybox.frag"), rgl::ShaderType::Fragment),
    ]);

    let cube_shader = learnopenrgl::utils::Program::new(&[
        learnopenrgl::utils::Shader::new(include_str!("cube.vert"), rgl::ShaderType::Vertex),
        learnopenrgl::utils::Shader::new(include_str!("cube.frag"), rgl::ShaderType::Fragment),
    ]);

    {
        let projection = glm::perspective(
            window.size().0 as f32 / window.size().1 as f32,
            45.0f32.to_radians(),
            0.1,
            100.0,
        );

        skybox_shader.enable();
        skybox_shader.set_uniform_i32("fragment_texture", 0);
        skybox_shader.set_uniform_mat4_multi(
            "projection",
            rgl::MatrixOrderMajor::Column,
            *projection.as_ref(),
        );

        cube_shader.enable();
        cube_shader.set_uniform_i32("fragment_texture", 0);
        cube_shader.set_uniform_mat4_multi(
            "projection",
            rgl::MatrixOrderMajor::Column,
            *projection.as_ref(),
        );

        rgl::use_program(rgl::Program::default());
        assert_eq!(rgl::get_error(), rgl::Error::NoError);
    }

    let skybox_vao = load_vertex_array(get_cube_data(), false);
    let cube_vao = load_vertex_array(get_cube_data(), true);
    assert_eq!(rgl::get_error(), rgl::Error::NoError);

    // ============================================================================================

    let mut camera = learnopenrgl::utils::Camera::new();
    let mut frame = learnopenrgl::utils::Frame::new_now();

    camera.set_position(0.0, 0.0, 5.0);

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
                event => learnopenrgl::utils::process_sdl_event(&mut camera, event),
            }
        }

        camera.update_position(frame.last_frame_duration(), 5.0);

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

        rgl::depth_func(rgl::CompareFunc::Less);
        cube_shader.enable();
        cube_shader.set_uniform_mat4_multi(
            "view",
            rgl::MatrixOrderMajor::Column,
            camera.calculate_view(),
        );

        rgl::bind_vertex_array(cube_vao);
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, cube_texture);
        rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
        assert_eq!(rgl::get_error(), rgl::Error::NoError);

        rgl::depth_func(rgl::CompareFunc::LessOrEqual);
        skybox_shader.enable();
        skybox_shader.set_uniform_mat4_multi(
            "view",
            rgl::MatrixOrderMajor::Column,
            *glm::mat3_to_mat4(&glm::mat4_to_mat3(&glm::Mat4::from(
                camera.calculate_view(),
            )))
            .as_ref(),
        );

        rgl::bind_vertex_array(skybox_vao);
        rgl::bind_texture(rgl::TextureBindingTarget::CubeMap, skybox_texture);
        rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
        assert_eq!(rgl::get_error(), rgl::Error::NoError);

        window.gl_swap_window();
    }

    Ok(())
}

fn load_vertex_array(data: &[f32], with_coords: bool) -> rgl::VertexArray {
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

    if with_coords {
        rgl::enable_vertex_attrib_array(1);
        rgl::vertex_attrib_float_pointer(
            1,
            rgl::VertexAttribSize::Double,
            rgl::VertexAttribFloatType::F32,
            false,
            stride,
            3 * std::mem::size_of::<f32>() as u64,
        );
    }

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
