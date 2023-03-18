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
    let window = video
        .window("LearnOpenGL: Light Maps Ex - Emission", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    rgl::enable(rgl::Capability::DepthTest);

    let cube_shader_program = {
        let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex);
        rgl::shader_source(vertex_shader, include_str!("cube.vert"));
        rgl::compile_shader(vertex_shader);
        assert!(rgl::get_shader_compile_status(vertex_shader));

        let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment);
        rgl::shader_source(fragment_shader, include_str!("cube.frag"));
        rgl::compile_shader(fragment_shader);
        assert!(rgl::get_shader_compile_status(fragment_shader));

        let shader_program = rgl::create_program();
        rgl::attach_shader(shader_program, vertex_shader);
        rgl::attach_shader(shader_program, fragment_shader);
        rgl::link_program(shader_program);
        assert!(rgl::get_program_link_status(shader_program));

        shader_program
    };

    let light_shader_program = {
        let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex);
        rgl::shader_source(vertex_shader, include_str!("light.vert"));
        rgl::compile_shader(vertex_shader);
        assert!(rgl::get_shader_compile_status(vertex_shader));

        let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment);
        rgl::shader_source(fragment_shader, include_str!("light.frag"));
        rgl::compile_shader(fragment_shader);
        assert!(rgl::get_shader_compile_status(fragment_shader));

        let shader_program = rgl::create_program();
        rgl::attach_shader(shader_program, vertex_shader);
        rgl::attach_shader(shader_program, fragment_shader);
        rgl::link_program(shader_program);
        assert!(rgl::get_program_link_status(shader_program));

        shader_program
    };

    let mut vao = Default::default();
    rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));

    let mut vbo = Default::default();
    rgl::gen_buffers(std::slice::from_mut(&mut vbo));

    rgl::bind_vertex_array(vao);
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, vbo);
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        &[
            // front
            [-0.5f32, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // left
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // top
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            // right
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            // bottom
            [0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            // back
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, -0.5],
        ],
        rgl::BufferUsageFrequency::Static,
        rgl::BufferUsageNature::Draw,
    );
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_float_pointer(
        0,
        rgl::VertexAttribSize::Triple,
        rgl::VertexAttribFloatType::F32,
        false,
        (std::mem::size_of::<f32>() * 3) as u64,
        0,
    );
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer(0));

    const EYE_POSITION: [f32; 3] = [1.5, 2.0, 5.0];

    let diffuse_texture = {
        rgl::use_program(cube_shader_program);

        let mut texture = rgl::Texture::default();
        rgl::gen_textures(std::slice::from_mut(&mut texture));

        rgl::active_texture(0);
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture);
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
            rgl::TextureMinFilter::Linear,
        );
        rgl::texture_target_mag_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMagFilter::Linear,
        );

        let image = image::open("./assets/container2.png")?.flipv();
        rgl::tex_image_2d(
            rgl::TextureBinding2DTarget::Image2D,
            0,
            rgl::TextureInternalFormat::RGB,
            image.width(),
            image.height(),
            rgl::TextureFormat::RGBA,
            rgl::TexturePixelType::U8,
            rgl::TextureData::Data(image.as_bytes()),
        );
        let loc = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.diffuse\0")?,
        );
        rgl::uniform_1i32(loc, 0);
        texture
    };

    let specular_texture = {
        rgl::use_program(cube_shader_program);

        let mut texture = rgl::Texture::default();
        rgl::gen_textures(std::slice::from_mut(&mut texture));

        rgl::active_texture(1);
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture);
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
            rgl::TextureMinFilter::Linear,
        );
        rgl::texture_target_mag_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMagFilter::Linear,
        );

        let image = image::open("./assets/container2_specular.png")?.flipv();
        rgl::tex_image_2d(
            rgl::TextureBinding2DTarget::Image2D,
            0,
            rgl::TextureInternalFormat::RGB,
            image.width(),
            image.height(),
            rgl::TextureFormat::RGBA,
            rgl::TexturePixelType::U8,
            rgl::TextureData::Data(image.as_bytes()),
        );
        let loc = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.specular\0")?,
        );
        rgl::uniform_1i32(loc, 1);
        texture
    };

    let emission_texture = {
        rgl::use_program(cube_shader_program);

        let mut texture = rgl::Texture::default();
        rgl::gen_textures(std::slice::from_mut(&mut texture));

        rgl::active_texture(2);
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, texture);
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
            rgl::TextureMinFilter::Linear,
        );
        rgl::texture_target_mag_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMagFilter::Linear,
        );

        let image = image::open("./assets/matrix.jpg")?.flipv();
        rgl::tex_image_2d(
            rgl::TextureBinding2DTarget::Image2D,
            0,
            rgl::TextureInternalFormat::RGB,
            image.width(),
            image.height(),
            rgl::TextureFormat::RGB,
            rgl::TexturePixelType::U8,
            rgl::TextureData::Data(image.as_bytes()),
        );
        let loc = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.emission\0")?,
        );
        rgl::uniform_1i32(loc, 2);
        texture
    };

    let cube_light_position = {
        rgl::use_program(cube_shader_program);

        let model = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
        );
        let view = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
        );
        let projection = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
        );
        let material_shininess = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.shininess\0")?,
        );
        let light_position = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.position\0")?,
        );
        let light_ambient = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.ambient\0")?,
        );
        let light_diffuse = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.diffuse\0")?,
        );
        let light_specular = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.specular\0")?,
        );
        let view_pos = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view_position\0")?,
        );

        rgl::uniform_matrix_4f32v_flat(
            model,
            rgl::MatrixOrderMajor::Row,
            &[from_glm_mat4(&glm::one())],
        );
        rgl::uniform_matrix_4f32v_flat(
            view,
            rgl::MatrixOrderMajor::Row,
            &[from_glm_mat4(&glm::look_at(
                &glm::make_vec3(&EYE_POSITION),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            ))],
        );
        rgl::uniform_matrix_4f32v_flat(
            projection,
            rgl::MatrixOrderMajor::Row,
            &[from_glm_mat4(&glm::perspective(
                window.size().0 as f32 / window.size().1 as f32,
                45.0f32.to_radians(),
                0.1,
                100.0,
            ))],
        );

        rgl::uniform_1f32(material_shininess, 32.0);
        rgl::uniform_3f32(light_ambient, 0.2, 0.2, 0.2);
        rgl::uniform_3f32(light_diffuse, 0.5, 0.5, 0.5);
        rgl::uniform_3f32(light_specular, 1.0, 1.0, 1.0);
        rgl::uniform_3f32(view_pos, EYE_POSITION[0], EYE_POSITION[1], EYE_POSITION[2]);

        light_position
    };

    let light_model = {
        rgl::use_program(light_shader_program);

        let model = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
        );
        let view = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
        );
        let projection = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
        );

        rgl::uniform_matrix_4f32v_flat(
            model,
            rgl::MatrixOrderMajor::Row,
            &[from_glm_mat4(&glm::one())],
        );
        rgl::uniform_matrix_4f32v_flat(
            view,
            rgl::MatrixOrderMajor::Row,
            &[from_glm_mat4(&glm::look_at(
                &glm::make_vec3(&EYE_POSITION),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            ))],
        );
        rgl::uniform_matrix_4f32v_flat(
            projection,
            rgl::MatrixOrderMajor::Row,
            &[from_glm_mat4(&glm::perspective(
                window.size().0 as f32 / window.size().1 as f32,
                45.0f32.to_radians(),
                0.1,
                100.0,
            ))],
        );
        model
    };

    // ============================================================================================

    rgl::clear_colour(0.1, 0.1, 0.1, 0.1);

    let mut event_pump = sdl.event_pump().unwrap();
    let start_time_point = std::time::Instant::now();
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

        let frame_time_point = std::time::Instant::now();
        let total_duration = frame_time_point - start_time_point;

        let light_position =
            glm::rotate_y_vec3(&glm::vec3(3.0, 1.5, 0.0), total_duration.as_secs_f32());

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

        {
            rgl::use_program(cube_shader_program);

            rgl::active_texture(0);
            rgl::bind_texture(rgl::TextureBindingTarget::Image2D, diffuse_texture);

            rgl::active_texture(1);
            rgl::bind_texture(rgl::TextureBindingTarget::Image2D, specular_texture);

            rgl::active_texture(2);
            rgl::bind_texture(rgl::TextureBindingTarget::Image2D, emission_texture);

            rgl::bind_vertex_array(vao);
            rgl::uniform_3f32v(cube_light_position, &[*light_position.as_ref()]);
            rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
        }

        {
            rgl::use_program(light_shader_program);
            rgl::uniform_matrix_4f32v_flat(
                light_model,
                rgl::MatrixOrderMajor::Row,
                &[from_glm_mat4(&glm::scale(
                    &glm::translate(&glm::one(), &light_position),
                    &glm::vec3(0.2, 0.2, 0.2),
                ))],
            );
            rgl::bind_vertex_array(vao);
            rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
        }

        window.gl_swap_window();
    }

    Ok(())
}

fn from_glm_mat4(mat: &glm::Mat4) -> [f32; 16] {
    #[rustfmt::skip]
    return [
        mat.m11, mat.m12, mat.m13, mat.m14,
        mat.m21, mat.m22, mat.m23, mat.m24,
        mat.m31, mat.m32, mat.m33, mat.m34,
        mat.m41, mat.m42, mat.m43, mat.m44,
    ];
}
